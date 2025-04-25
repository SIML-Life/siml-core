mod helpers;

use helpers::agent_stub::run_agent_stub;
use siml_core::socket::{start_server_with_dispatcher, Dispatcher};
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[tokio::test]
async fn test_multiple_agents_connect() {
    const NUM_AGENTS: usize = 10;

    // 🛠️ 1. Create shared dispatcher
    let dispatcher = Arc::new(Dispatcher::new());

    // 🛠️ 2. Start server with our dispatcher
    let server_task = tokio::spawn(start_server_with_dispatcher(
        "127.0.0.1:6000",
        dispatcher.clone(),
        false
    ));

    // Wait a moment for server to be ready
    sleep(Duration::from_millis(200)).await;

    // 🧠 3. Launch agent stubs
    let mut handles = vec![];

    for id in 1..=NUM_AGENTS {
        handles.push(tokio::spawn(run_agent_stub(id as u32)));
    }

    // Allow agents time to connect and send
    sleep(Duration::from_secs(2)).await;

    // Await all agent tasks
    for h in handles {
        let _ = h.await;
    }

    // 🛑 4. Clean shutdown: abort server task
    server_task.abort();

    println!("✅ Multi-agent test completed");
}
