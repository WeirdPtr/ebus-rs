use async_trait::async_trait;

use super::event::Event;

#[async_trait]
pub trait EventBusSubscriber {
    type InputDataType;

    async fn on_event_publish(&mut self, event: &Event<Self::InputDataType>);
}
