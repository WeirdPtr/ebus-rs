use ebus::{async_subscriber, EventBus, Subscriber};

#[derive(Default)]
pub struct ExampleSubscriber;

#[async_subscriber]
impl Subscriber for ExampleSubscriber {
    type Input = String;

    async fn on_event_publish(&mut self, event: &Self::Input) {
        println!("Received Data: {:#?}", event);
    }
}

#[tokio::main]
async fn main() {
    // Create a new event bus
    let mut event_bus: EventBus<String> = EventBus::default();

    // Create a new subscriber and subscribe it to the event bus
    let subscriber = ExampleSubscriber::default();
    event_bus.subscribe(subscriber);

    // Create an event and queue it for processing
    let event_data = "Hello World!".to_owned();
    event_bus.queue_and_process(event_data.into()).await;
}
