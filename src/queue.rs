use crate::message::Message;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

pub struct MessageQueue<T: Clone> {
    pub queue: Arc<RwLock<VecDeque<Message<T>>>>,
}

impl<T: Clone> Default for MessageQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> MessageQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn send_message(&self, message: Message<T>) {
        let mut queue = self.queue.write().unwrap(); // Lock for writing.
        queue.push_back(message);
    }

    pub fn receive_message(&self) -> Option<Message<T>> {
        let mut queue = self.queue.write().unwrap(); // Lock for writing since we're popping.
        queue.pop_front()
    }

    pub fn delete_message(&self, message_id: u64) {
        let mut queue = self.queue.write().unwrap(); // Lock for writing since we're modifying.
        queue.retain(|msg| msg.id != message_id);
    }

    pub fn filter_by_attribute(&self, key: &str, value: &str) -> Vec<Message<T>> {
        let queue = self.queue.read().unwrap(); // Lock for reading.
        queue
            .iter()
            .filter(|msg| {
                if let Some(attrs) = &msg.attributes {
                    attrs.get(key) == Some(&value.to_string())
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_access() {
        let queue: Arc<MessageQueue<String>> = Arc::new(MessageQueue::new());

        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let message = Message {
                id: 1,
                body: "Concurrent message".to_string(),
                attributes: None,
            };
            queue_clone.send_message(message);
        });

        handle.join().unwrap();

        // Verify the message is in the queue.
        assert_eq!(
            queue.receive_message(),
            Some(Message {
                id: 1,
                body: "Concurrent message".to_string(),
                attributes: None,
            })
        );
    }
}
