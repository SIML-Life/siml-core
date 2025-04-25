use std::sync::Arc;
use siml_core::socket;

#[tokio::main]
async fn main() {
    let dispatcher = Arc::new(siml_core::socket::Dispatcher::new());
    socket::start_server("127.0.0.1:6000", dispatcher).await;
}