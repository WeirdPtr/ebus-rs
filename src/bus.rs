use crate::Subscriber;

pub struct EventBus<T>
where
    T: Send + Sync + 'static,
{
    events: Vec<T>,
    pub subscribers: Vec<Box<dyn Subscriber<Input = T>>>,
}

impl<T: Send + Sync> EventBus<T> {
    pub const fn new() -> Self {
        Self {
            events: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn queue(&mut self, message: T) {
        self.events.push(message);
    }

    pub fn subscribe<S: Subscriber<Input = T> + 'static>(&mut self, listener: S)
    where
        S: Into<Box<S>>,
    {
        self.subscribers.push(listener.into());
    }

    pub fn clear_queue(&mut self) {
        self.events.clear();
    }

    #[inline]
    pub async fn process_queue(&mut self) {
        for event in self.events.drain(..) {
            for listener in &mut self.subscribers {
                listener.on_event_publish(&event).await;
            }
        }
    }

    pub async fn queue_and_process(&mut self, message: T) {
        self.queue(message);
        self.process_queue().await;
    }

    pub async fn force_process_single(&mut self, message: T) {
        self.process_single(message).await;
    }

    #[inline]
    async fn process_single(&mut self, message: T) {
        for listener in &mut self.subscribers {
            listener.on_event_publish(&message).await;
        }
    }
}

impl<T: Send + Sync> Default for EventBus<T> {
    fn default() -> Self {
        Self::new()
    }
}
