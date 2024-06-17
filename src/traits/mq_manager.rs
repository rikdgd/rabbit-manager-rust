use std::error::Error;
use std::future::Future;
use crate::traits::mq_message::MqMessage;


pub trait MqManager<T>
    where
        T: MqMessage
{
    fn send_message(&self, message: T) -> impl Future<Output=Result<(), Box<dyn Error>>>;
    fn read_next_message(&self) -> impl Future<Output=Option<T>>;
    fn run_handler_function(&mut self, queue_name: &str, handler_fn: impl Fn(T)) -> impl Future<Output=()>;
    fn close_connection(&mut self) -> impl Future<Output=Result<(), Box<dyn Error>>>;
}
