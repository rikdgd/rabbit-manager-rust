use crate::traits::mq_message::MqMessage;

pub struct BasicMessage {
    message: String,
}

impl MqMessage for BasicMessage {
    fn from_str(message: &str) -> Self {
        BasicMessage { message: message.to_string() }
    }

    fn as_string(&self) -> String {
        self.message.clone()
    }
}
