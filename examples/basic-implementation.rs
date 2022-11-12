use async_trait::async_trait;
use ebus::{bus::EventBus, event::Event, subscriber::EventBusSubscriber};

#[derive(Clone, Debug)]
pub struct ExampleData {
    pub data: String,
}

pub struct ExampleDataSubscriber {
    data: ExampleData,
}

#[async_trait]
impl EventBusSubscriber for ExampleDataSubscriber {
    type InputDataType = ExampleData;

    async fn on_event_publish(&mut self, event: &Event<Self::InputDataType>) {
        let data = event.data.to_owned();

        println!("Received Data: {:#?}", data);

        self.data = data;
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
        .publish_and_process(Event::new(ExampleData {
            data: "I am Data".to_owned(),
        }))
        .await;
}
