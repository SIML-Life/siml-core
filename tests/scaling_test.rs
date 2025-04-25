mod helpers;

use helpers::agent_stub::run_agent_stub;
use siml_core::socket::{Dispatcher, print_metrics, start_server_with_dispatcher};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_scaling_stress() {
    const NUM_AGENTS: usize = 2500;

    // 🛠 Shared dispatcher for server and metrics
    let dispatcher = Arc::new(Dispatcher::new());

    // 🛠 Start the server with this dispatcher
    let dispatcher_server = dispatcher.clone();
    tokio::spawn(async move {
        start_server_with_dispatcher("127.0.0.1:6000", dispatcher_server, false).await;
    });

    // 🔥 Start metrics printing (using the same dispatcher)
    tokio::spawn(print_metrics(dispatcher.clone()));

    // ✈️ Launch agents
    let mut handles = vec![];
    for id in 1..=NUM_AGENTS {
        handles.push(tokio::spawn(run_agent_stub(id as u32)));
    }

    // Allow agents time to connect/send
    sleep(Duration::from_secs(10)).await;

    // Await all agent tasks
    for h in handles {
        let _ = h.await;
    }

    println!("✅ Scaling stress test completed");
}
