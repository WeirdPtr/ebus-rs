pub struct Event<T> {
    pub data: T,
}

impl<T> Event<T> {
    pub const fn new(data: T) -> Self {
        Self { data }
    }

    pub const fn data_ref(&self) -> &T {
        &self.data
    }
}

impl<T> From<T> for Event<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}
