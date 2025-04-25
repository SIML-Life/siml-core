mod helpers;

use helpers::agent_stub::run_agent_stub;
use helpers::env_stub::run_env_stub;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_full_core_roundtrip() {
    // NOTE: Server must be running!
    let a = tokio::spawn(run_agent_stub(1));  // <-- Pass agent ID here
    let b = tokio::spawn(run_env_stub());

    sleep(Duration::from_secs(1)).await;

    let _ = tokio::try_join!(a, b);
}
