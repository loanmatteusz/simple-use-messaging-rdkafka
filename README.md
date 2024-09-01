# Kafka Messaging with Rust using Tokio and rdkafka

This project is a simple application to demonstrate basic messaging concepts with Kafka using Rust, Tokio, and the rdkafka library. It includes a producer that sends messages to a Kafka topic and a consumer that consumes those messages.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)

## How to Run

### 1. Clone the Repository

```bash
git clone https://github.com/loanmatteusz/simple-use-messaging-rdkafka.git
cd simple-use-messaging-rdkafka
```

### 2. Start the Containers with Docker Compose

Before running the application, you need to start Kafka and Zookeeper using Docker Compose:

````bash
docker-compose up -d
````
### 3. Run the Application

With the Kafka and Zookeeper containers running, you can execute the Rust application:

```bash
cargo run --quiet
```
This will send a message to the test-topic and consume that same message.

### Project Structure

- producer.rs: Contains the code to create a producer and send messages to Kafka.
- consumer.rs: Contains the code to create a consumer and read messages from Kafka.
- main.rs: The entry point of the application, which creates a producer to send a message and starts a consumer to read it.

### Configuration

- Kafka Topic: `test-topic`
- Bootstrap Servers: `localhost:9092`
- Consumer Group: `test-group`

### Docker Compose

The docker-compose.yml sets up two services:

- zookeeper: Necessary for Kafka to operate.
- kafka: The Kafka broker that will be used to send and receive messages.

### Rust Dependencies

- [Tokio](https://docs.rs/tokio/latest/tokio/) for async runtime.
- [rdkafka](https://docs.rs/rdkafka/latest/rdkafka/) for Kafka integration.

### Troubleshooting
#### Connection Errors

If you encounter connection errors, such as Connection refused, ensure that Kafka is running correctly on port 9092 and that localhost is the correct address.
````bash
docker-compose down
docker-compose up -d
````
### About
This project is a study and was written following [this video](https://youtu.be/yAg5JDs4EKI) made by the YouTube user `Semicolon`.
