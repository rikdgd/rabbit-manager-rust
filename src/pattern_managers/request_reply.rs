use std::error::Error;
use std::io::ErrorKind;
use lapin::{
    options::*,
    types::FieldTable,
    BasicProperties,
    Connection,
    ConnectionProperties,
    Channel,
};
use futures::StreamExt;
use crate::pattern_managers::manager_trait::PatternManager;



pub struct RequestReplyManager {
    connection_closed: bool,
    queue_name: String,
    connection: Connection,
    channel: Channel,
}


#[allow(unused)]
impl RequestReplyManager {
    pub async fn new(address: &str, queue_name: &str) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::connect(address, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel.queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default()
        ).await?;

        Ok(Self {
            connection_closed: false,
            queue_name: queue_name.to_string(),
            connection,
            channel,
        })
    }

    pub async fn exchange_message(&mut self, message: &str) -> Result<String, Box<dyn Error>> {
        self.send_message(message.to_string()).await?;
        let received = self.await_message().await?;
        Ok(received)
    }

    async fn send_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        let payload = message.as_bytes();
        self.channel.basic_publish(
            "",
            &self.queue_name,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default()
        ).await?;

        Ok(())
    }

    async fn await_message(&mut self) -> Result<String, Box<dyn Error>> {
        let consumer = &mut self.channel
            .basic_consume(
                self.queue_name(),
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to create queue consumer.");

        if let Ok(delivery) = consumer.next().await.expect("No message found.") {
            delivery.ack(BasicAckOptions::default()).await?;
            let message = String::from_utf8(delivery.data)?;
            return Ok(message);
        }

        self.close_connection().await?;
        Err(Box::new(std::io::Error::new(
            ErrorKind::ConnectionAborted,
            "Closed the connection to RabbitMQ, since no message was received."
        )))
    }
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }
    
    pub fn connection_closed(&self) -> bool {
        self.connection_closed
    }
}


impl PatternManager for RequestReplyManager {
    async fn close_connection(&mut self) -> Result<(), Box<dyn Error>> {
        self.channel.close(0, "").await?;
        self.connection.close(0, "").await?;
        self.connection_closed = true;
        Ok(())
    }
}


impl Drop for RequestReplyManager {
    fn drop(&mut self) {
        if !self.connection_closed {
            panic!("Failed to close connection to queue: {}", self.queue_name);
        }
    }
}
