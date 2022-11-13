#[cfg(feature = "uuid")]
use uuid::Uuid;

pub struct Event<T> {
    #[cfg(feature = "uuid")]
    uuid: String,
    pub data: T,
}

impl<T> Event<T> {
    pub fn new(data: T) -> Event<T> {
        Event {
            #[cfg(feature = "uuid")]
            uuid: Uuid::new_v4().to_string(),
            data,
        }
    }

    pub fn get_data(&self) -> &T {
        return &self.data;
    }

    #[cfg(feature = "uuid")]
    pub fn get_uuid(&self) -> String {
        return self.uuid.clone();
    }
}

impl<T> From<T> for Event<T> {
    fn from(data: T) -> Self {
        Event::new(data)
    }
}
