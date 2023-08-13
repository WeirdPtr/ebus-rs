use ebus::{async_subscriber, Event, EventBus, EventBusSubscriber};

#[derive(Debug, Clone)]
pub struct ExampleData {
    pub data: String,
}

#[derive(Default)]
pub struct ExampleDataSubscriber;

#[async_subscriber]
impl EventBusSubscriber for ExampleDataSubscriber {
    type InputDataType = ExampleData;

    async fn on_event_publish(&mut self, event: &Event<Self::InputDataType>) {
        println!("Received Data: {:#?}", event.data_ref());
    }
}

#[tokio::main]
async fn main() {
    let mut example_bus: EventBus<ExampleData> = EventBus::default();

    let subscriber = ExampleDataSubscriber::default();

    example_bus.subscribe(subscriber);

    let event = Event::new(ExampleData {
        data: "Hello World!".to_owned(),
    });

    example_bus.queue_and_process(event).await;
}
