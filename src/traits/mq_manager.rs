use std::io::Result;
use crate::traits::mq_message::MqMessage;


pub trait MqManager<T>
    where
        T: MqMessage
{
    fn send_message(&self, message: T) -> Result<()>;
    fn read_next_message(&self) -> Option<T>;
    fn attach_handler_function(&mut self, queue_name: &str, handler_fn: impl Fn());
}
