use crate::producer::produce;

mod producer;
mod consumer;

#[tokio::main]
async fn main() {
    let producer = producer::create();

    produce(producer, String::from("Hello World, I'm testing!")).await;

    consumer::start().await;
}
