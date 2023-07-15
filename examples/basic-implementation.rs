use ebus::{Event, EventBus, EventBusSubscriber};

#[derive(Debug, Clone)]
pub struct ExampleData {
    pub data: String,
}

pub struct ExampleDataSubscriber {
    data: ExampleData,
}

#[async_trait::async_trait]
impl EventBusSubscriber for ExampleDataSubscriber {
    type InputDataType = ExampleData;

    async fn on_event_publish(&mut self, event: &Event<Self::InputDataType>) {
        println!("Received Data: {:#?}", event.data_ref());

        self.data = event.data.clone();
    }
}

#[tokio::main]
async fn main() {
    let mut example_bus: EventBus<ExampleData> = EventBus::default();

    let subscriber = ExampleDataSubscriber {
        data: ExampleData {
            data: "".to_owned(),
        },
    };

    example_bus.subscribe(subscriber);

    example_bus
        .queue_and_publish(Event::new(ExampleData {
            data: "I am Data".to_owned(),
        }))
        .await;
}
