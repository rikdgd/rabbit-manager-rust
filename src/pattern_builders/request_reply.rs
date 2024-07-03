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
use crate::pattern_builders::builder_trait::PatternBuilder;


pub struct RequestReplyBuilder {
    pub connection_closed: bool,
    address: String,
    queue_name: String,
    connection: Connection,
    channel: Channel,
}
impl RequestReplyBuilder {
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
            address: address.to_string(),
            queue_name: queue_name.to_string(),
            connection,
            channel,
        })
    }

    pub async fn exchange_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        todo!()
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

        if let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                delivery.ack(BasicAckOptions::default()).await.expect("Failed to acknowledge delivery");
                let message = String::from_utf8(delivery.data).expect("Failed to read data from delivery.");
                return Ok(message);
            }
        }
        
        self.close_connection().await.expect("Failed to close connection after also failing to receive message.");
        Err(Box::new(std::io::Error::new(
            ErrorKind::ConnectionAborted, 
            "Closed the connection to RabbitMQ, since no message was received."
        )))
    }
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }
}

impl PatternBuilder for RequestReplyBuilder {
    async fn close_connection(&mut self) -> Result<(), Box<dyn Error>> {
        self.channel.close(0, "").await?;
        self.connection.close(0, "").await?;
        self.connection_closed = true;
        Ok(())
    }
}

impl Drop for RequestReplyBuilder {
    fn drop(&mut self) {
        if !self.connection_closed {
            panic!("Failed to close connection to queue: {}", self.queue_name);
        }
    }
}
