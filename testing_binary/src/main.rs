use std::error::Error;
use rabbit_manager;
use rabbit_manager::basic_message::BasicMessage;
use rabbit_manager::single_queue_manager::SingleQueueManager;
use rabbit_manager::traits::{
    mq_manager::MqManager,
    mq_message::MqMessage,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = SingleQueueManager::new(
        "amqp://localhost:5672", 
        "test123"
    ).await?;
    
    manager.send_message(BasicMessage::from_str("hello world!")).await?;
    manager.send_message(BasicMessage::from_str("how are you?")).await?;
    manager.send_message(BasicMessage::from_str("great to hear!")).await?;
    manager.send_message(BasicMessage::from_str("bye world, i guess?")).await?;
    
    
    fn handler_fn(next_message: BasicMessage) {
        println!("received message: {}", next_message.as_string());
    }
    
    manager.run_handler_function("", handler_fn).await;
    
    manager.close_connection().await?;
    Ok(())
}
