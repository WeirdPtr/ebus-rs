<div align="center">
  <a href="https://github.com/WeirdPtr/ebus/">
    <img src="img/logo.png" alt="Logo" width="150" height="150">
  </a>
</div>

<div id="top"></div>
<br />
<div align="center">
<h3 align="center">ebus</h3>
  <p align="center">
    ebus is a simple Eventbus implementation written in Rust.
    <br />
    <br />
    <a href="https://github.com/WeirdPtr/ebus/issues">Report Bug</a>
    ·
    <a href="https://github.com/WeirdPtr/ebus/issues">Request Feature</a>
  </p>
</div>

## About The Project

This is a small Eventbus implementation that can be used to asynchronously transmit data to subscribers of a EventBus instance.

### Built With

- [Rust](https://www.rust-lang.org/)
- [async-trait](https://crates.io/crates/async-trait/)

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Usage

- To build the lib run: `cargo build --release`
- To run the example run: `cargo run --example basic-implementation`

### Basic Example

```rust
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

    example_bus.force_process_single(event).await;
}

```

## Roadmap

See the [open issues](https://github.com/WeirdPtr/ebus/issues) for a full list of proposed features (and known issues).

## Contributing

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT or Apache 2 License. See [`LICENSE-MIT`](https://github.com/WeirdPtr/ebus/blob/master/LICENSE-MIT) or [`LICENSE-APACHE`](https://github.com/WeirdPtr/ebus/blob/master/LICENSE-APACHE) for more information.
