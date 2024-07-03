use std::error::Error;

pub trait PatternBuilder {
    async fn close_connection(&mut self) -> Result<(), Box<dyn Error>>;
}
