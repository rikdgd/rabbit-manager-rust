use std::error::Error;

pub trait PatternManager {
    async fn close_connection(&mut self) -> Result<(), Box<dyn Error>>;
}
