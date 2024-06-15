pub mod traits;
pub mod single_queue_manager;
pub mod basic_message;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::basic_message::BasicMessage;
    use crate::traits::mq_message::MqMessage;
    
    #[test]
    fn create_basic_message() {
        let message_text = "Hello world!";
        let message = BasicMessage::from_str(message_text);
        assert_eq!(message.as_string(), message_text.to_string())
    }
}
