use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};

pub async fn start() {
    let consumer: StreamConsumer = create();
    consume(consumer).await
}

fn create() -> StreamConsumer {
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .set("group.id", "test-group")
        .set("socket.timeout.ms", "4000");

    let consumer: StreamConsumer = config.create().expect("Failure to create consumer.");

    consumer
}

pub async fn consume(consumer: StreamConsumer) {
    consumer.subscribe(&["test-topic"]).expect("Con't subscribe");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                match message.payload_view::<str>() {
                    Some(Ok(msg)) => println!("Message consumed: {}", msg),
                    Some(Err(e)) => println!("Error parsing: {:?}", e),
                    None => println!("No message"),
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            },
            Err(e) => println!("{:?}", e)
        }
    }
}
