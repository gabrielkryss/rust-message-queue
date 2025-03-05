# rust message queue

This is a very simple in memory message queue library implementation.
The goal was to learn and understand the core
fundamental functions of message queue systems.

Simple features are implemented such as

- Generic Message Type `Message<T>`
- MessageQueue (FIFO)
- Visibility control
- Dead Letter Queue

## Example

`examples/example1.rs`

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rust_message_queue::{DeadLetterQueue, Message, MessageQueue, VisibilityControl};

fn main() {
    // Initialize shared components
    let queue: Arc<MessageQueue<String>> = Arc::new(MessageQueue::new());
    let visibility_control = Arc::new(Mutex::new(VisibilityControl::new()));
    let dlq: Arc<DeadLetterQueue<String>> = Arc::new(DeadLetterQueue::new(3));

    // Create producer threads
    let mut producer_handles = vec![];
    for producer_id in 1..=3 {
        let queue_clone = Arc::clone(&queue);

        let handle = thread::spawn(move || {
            for i in 1..=5 {
                let mut attributes = HashMap::new();
                attributes.insert("Producer".to_string(), format!("Producer-{}", producer_id));
                attributes.insert("Type".to_string(), "Task".to_string());

                let message = Message {
                    id: producer_id * 100 + i,
                    body: format!("Message {} from Producer {}", i, producer_id),
                    attributes: Some(attributes),
                };

                queue_clone.send_message(message.clone());
                println!("[Producer {}] Sent: {:?}", producer_id, message);
            }
        });
        producer_handles.push(handle);
    }

    // Create consumer threads
    let mut consumer_handles = vec![];
    for consumer_id in 1..=2 {
        let queue_clone = Arc::clone(&queue);
        let visibility_control_clone = Arc::clone(&visibility_control);
        let dlq_clone = Arc::clone(&dlq);

        let handle = thread::spawn(move || {
            for _ in 0..8 {
                if let Some(received) = queue_clone.receive_message() {
                    println!("[Consumer {}] Received: {:?}", consumer_id, received);

                    // Simulate visibility timeout (e.g., 5 seconds)
                    let mut vis_control = visibility_control_clone.lock().unwrap();
                    vis_control.set_visibility_timeout(received.id, Duration::from_secs(5));

                    // Simulate message processing
                    thread::sleep(Duration::from_secs(2));
                    if consumer_id % 2 == 0 {
                        // Simulate a processing failure
                        println!(
                            "[Consumer {}] Failed to process: {:?}",
                            consumer_id, received
                        );
                        dlq_clone.handle_failure(received.clone());
                    } else {
                        println!(
                            "[Consumer {}] Successfully processed: {:?}",
                            consumer_id, received
                        );
                    }
                } else {
                    println!("[Consumer {}] No messages available.", consumer_id);
                }
            }
        });
        consumer_handles.push(handle);
    }

    // Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Wait for all consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    // Print remaining messages in DLQ
    let dead_letters = dlq.get_dead_letters();
    println!("Dead letters: {:?}", dead_letters);
}
```
