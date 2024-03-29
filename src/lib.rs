#![allow(clippy::module_name_repetitions)]

pub(crate) mod bus;

#[cfg(feature = "async_subscriber")]
pub use async_trait::async_trait as async_subscriber;

pub type EventBus<T> = bus::EventBus<T>;

#[async_trait::async_trait]
pub trait Subscriber: Send + Sync + 'static {
    type Input;

    async fn on_event_publish(&mut self, event: &Self::Input);
}
  