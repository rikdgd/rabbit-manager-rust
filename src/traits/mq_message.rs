pub trait MqMessage {
    fn from_str(message: &str) -> Self;
    fn as_string(&self) -> String;
}
