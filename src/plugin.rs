use crate::socket::EnvelopeMessage;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> u32;
    fn is_agent(&self) -> bool;
    async fn handle_message(&mut self, msg: EnvelopeMessage);
}