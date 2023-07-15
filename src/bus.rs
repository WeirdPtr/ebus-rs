use crate::{event::Event, EventBusSubscriber};

pub struct EventBus<T>
where
    T: Send + Sync + 'static,
{
    events: Vec<Event<T>>,
    pub subscribers: Vec<Box<dyn EventBusSubscriber<InputDataType = T>>>,
}

impl<T: Send + Sync> EventBus<T> {
    pub const fn new() -> Self {
        Self {
            events: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn queue_event(&mut self, message: Event<T>) {
        self.events.push(message);
    }

    pub fn subscribe_boxed<S: EventBusSubscriber<InputDataType = T> + 'static>(
        &mut self,
        listener: Box<S>,
    ) {
        self.subscribers.push(listener);
    }

    pub fn subscribe<S: EventBusSubscriber<InputDataType = T> + 'static>(&mut self, listener: S) {
        self.subscribe_boxed(Box::new(listener));
    }

    pub fn clear_queue(&mut self) {
        self.events.clear();
    }

    pub async fn process_queue(&mut self) {
        for event in self.events.drain(..) {
            for listener in &mut self.subscribers {
                listener.on_event_publish(&event).await;
            }
        }
    }

    pub async fn queue_and_publish(&mut self, message: Event<T>) {
        self.queue_event(message);
        self.process_queue().await;
    }
}

impl<T: Send + Sync> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}
