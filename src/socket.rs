use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use flatbuffers::root;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

use procfs::process::Process;
use crate::generated::handshake::{root_as_handshake, Role};
use crate::generated::message::MessageEnvelope;

#[derive(Debug, Default)]
pub struct MetricsSnapshot {
    pub peak_mem_used_mb: u64,
    pub peak_cpu_usage: f32,
}

pub type AgentId = u32;

#[derive(Debug)]
pub enum EnvelopeMessage {
    FromAgent((Vec<u8>, MessageEnvelope<'static>)),
    FromEnv((Vec<u8>, MessageEnvelope<'static>)),
}

pub struct Dispatcher {
    pub registry: Arc<tokio::sync::Mutex<HashMap<AgentId, mpsc::Sender<EnvelopeMessage>>>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&self, id: AgentId, tx: mpsc::Sender<EnvelopeMessage>) {
        self.registry.lock().await.insert(id, tx);
    }

    pub async fn dispatch(&self, msg: EnvelopeMessage, _is_agent: bool) {
        let agent_id = match &msg {
            EnvelopeMessage::FromAgent((_, env)) => env.agent_id(),
            EnvelopeMessage::FromEnv((_, env)) => env.agent_id(),
        };

        if let Some(tx) = self.registry.lock().await.get(&agent_id) {
            let _ = tx.send(msg).await;
        }
    }
}

/// Starts the server normally (show handshakes)
pub async fn start_server(addr: &str, dispatcher: Arc<Dispatcher>) {
    start_server_with_dispatcher(addr, dispatcher, true).await;
}

/// Starts the server with a Dispatcher (test mode can disable handshake prints)
pub async fn start_server_with_dispatcher(addr: &str, dispatcher: Arc<Dispatcher>, show_handshakes: bool) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let dispatcher = dispatcher.clone();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 4096];

            // Step 1: Read handshake
            let n = socket.read(&mut buf).await.unwrap();
            let handshake = root_as_handshake(&buf[..n]).expect("Invalid handshake");
            let agent_id = handshake.agent_id();
            let is_agent = handshake.role() == Role::Agent;

            if show_handshakes {
                println!(
                    "🤝 Handshake from {} ({})",
                    agent_id,
                    handshake.agent_type().unwrap()
                );
            }

            let (tx, mut _rx) = mpsc::channel(32);
            dispatcher.register(agent_id, tx).await;

            // Step 2: Read messages
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => break, // EOF
                    Ok(n) => n,
                    Err(_) => break,
                };

                let owned_bytes = buf[..n].to_vec();

                if let Ok(env) = root::<MessageEnvelope>(&owned_bytes) {
                    let env: MessageEnvelope<'static> = unsafe { std::mem::transmute(env) };

                    let msg = if is_agent {
                        EnvelopeMessage::FromAgent((owned_bytes, env))
                    } else {
                        EnvelopeMessage::FromEnv((owned_bytes, env))
                    };

                    dispatcher.dispatch(msg, is_agent).await;
                } else {
                    eprintln!("❌ Invalid envelope");
                }
            }
        });
    }
}

/// Print live memory, CPU, and connection metrics
pub async fn print_metrics(dispatcher: Arc<Dispatcher>) {
    let pid = std::process::id() as i32;
    let process = Process::new(pid).unwrap();

    let mut last_total_cpu = {
        let stat = process.stat().unwrap();
        stat.utime + stat.stime
    };
    let mut last_time = Instant::now();

    let mut max_mem_rss_kb = 0;
    let mut max_cpu_usage = 0.0;

    loop {
        if let Ok(statm) = process.statm() {
            let page_size_kb = 4096 / 1024; // 4KB pages
            let mem_rss_kb = statm.resident * page_size_kb;

            if mem_rss_kb > max_mem_rss_kb {
                max_mem_rss_kb = mem_rss_kb;
            }

            let now = Instant::now();
            if let Ok(stat) = process.stat() {
                let total_cpu = stat.utime + stat.stime;
                let delta_cpu = total_cpu.saturating_sub(last_total_cpu);
                let delta_time = now.duration_since(last_time).as_secs_f64();
                let cpu_usage = (delta_cpu as f64 / delta_time) / num_cpus::get() as f64 * 100.0;

                if cpu_usage > max_cpu_usage {
                    max_cpu_usage = cpu_usage;
                }

                last_total_cpu = total_cpu;
                last_time = now;

                let agents = dispatcher.registry.lock().await.len();

                println!(
                    "📊 RSS: {} MB (Peak: {} MB) | CPU: {:.2}% (Peak: {:.2}%) | 🧠 Agents Connected: {}",
                    mem_rss_kb / 1024,
                    max_mem_rss_kb / 1024,
                    cpu_usage,
                    max_cpu_usage,
                    agents
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
