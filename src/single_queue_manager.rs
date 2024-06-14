use std::error::Error;
use lapin::{
    options::*,
    types::FieldTable,
    BasicProperties,
    Connection,
    ConnectionProperties,
    Channel,
};
use futures::StreamExt;

use crate::basic_message::BasicMessage;
use crate::traits::mq_manager::MqManager;
use crate::traits::mq_message::MqMessage;



pub struct SingleQueueManager {
    pub connection_closed: bool,
    address: String,
    queue_name: String,
    connection: Connection,
    channel: Channel,
}

impl SingleQueueManager {
    pub async fn new(address: &str, queue_name: &str) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::connect(address, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel.queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default()
        ).await?;

        Ok(
            Self {
                connection_closed: false,
                address: address.to_string(),
                queue_name: queue_name.to_string(),
                connection,
                channel,
            }
        )
    }

    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub async fn close_connection(&mut self) -> Result<(), Box<dyn Error>> {
        self.channel.close(0, "").await?;
        self.connection.close(0, "").await?;
        self.connection_closed = true;
        Ok(())
    }
}

impl MqManager<BasicMessage> for SingleQueueManager {
    async fn send_message(&self, message: BasicMessage) -> Result<(), Box<dyn Error>> {
        let message_string = message.as_string();
        let payload = message_string.as_bytes();
        self.channel.basic_publish(
            "",
            &self.queue_name,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default()
        ).await?;

        Ok(())
    }

    async fn read_next_message(&self) -> Option<BasicMessage> {
        let consumer = &mut self.channel
            .basic_consume(
                self.queue_name(),
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        if let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                delivery.ack(BasicAckOptions::default()).await.expect("Failed to acknowledge delivery");
                let message = BasicMessage::from_str(
                    std::str::from_utf8(&delivery.data).expect("Failed to read data from delivery.")
                );
                
                return Some(message);
            }
        }

        None
    }

    fn attach_handler_function(&mut self, queue_name: &str, handler_fn: impl Fn()) {
        todo!()
    }
}

impl Drop for SingleQueueManager {
    fn drop(&mut self) {
        if !self.connection_closed {
            panic!("Failed to close connection to queue: {}", self.queue_name);
        }
    }
}

// tutorial here: https://github.com/rabbitmq/rabbitmq-tutorials/tree/main/rust-lapin/src/bin
