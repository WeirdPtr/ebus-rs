#![allow(clippy::module_name_repetitions)]

pub(crate) mod bus;
pub(crate) mod event;

pub use async_trait::async_trait as async_subscriber;

pub type EventBus<T> = bus::EventBus<T>;
pub type Event<T> = event::Event<T>;

#[async_subscriber]
pub trait EventBusSubscriber: Send + Sync + 'static {
    type InputDataType;

    async fn on_event_publish(&mut self, event: &Event<Self::InputDataType>);
}
