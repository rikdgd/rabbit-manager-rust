use crate::basic_message::BasicMessage;
use crate::traits::mq_manager::MqManager;
use crate::traits::mq_message::MqMessage;



pub struct SingleQueueManager {

}

impl MqManager<BasicMessage> for SingleQueueManager {
    fn send_message(&self, message: impl MqMessage) -> std::io::Result<()> {
        todo!()
    }

    fn read_next_message(&self) -> Option<BasicMessage> {
        todo!()
    }

    fn attach_handler_function(&mut self, queue_name: &str, handler_fn: impl Fn()) {
        todo!()
    }
}

// tutorial here: https://github.com/rabbitmq/rabbitmq-tutorials/tree/main/rust-lapin/src/bin
