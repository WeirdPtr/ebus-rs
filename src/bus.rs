#[cfg(feature = "uuid")]
use uuid::Uuid;

use super::{event::Event, subscriber::EventBusSubscriber};

pub struct EventBus<T> {
    #[cfg(feature = "uuid")]
    uuid: String,
    events: Vec<Event<T>>,
    pub subscribers: Vec<Box<dyn EventBusSubscriber<InputDataType = T>>>,
}

impl<T> EventBus<T> {
    pub fn new() -> EventBus<T> {
        EventBus {
            #[cfg(feature = "uuid")]
            uuid: Uuid::new_v4().to_string(),
            events: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub async fn process_event_queue(&mut self) {
        for event in self.events.drain(..) {
            for listener in self.subscribers.iter_mut() {
                listener.on_event_publish(&event).await;
            }
        }
    }

    pub fn publish(&mut self, message: Event<T>) {
        self.events.push(message);
    }

    pub async fn publish_and_process(&mut self, message: Event<T>) {
        self.events.push(message);
        self.process_event_queue().await;
    }

    pub fn subscribe<R: EventBusSubscriber<InputDataType = T> + 'static>(&mut self, listener: R) {
        self.subscribers.push(Box::new(listener));
    }

    pub fn clear_event_queue(&mut self) {
        self.events.clear();
    }

    #[cfg(feature = "uuid")]
    pub fn get_uuid(&self) -> String {
        return self.uuid.clone();
    }
}

impl<T> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}
