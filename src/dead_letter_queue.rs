use crate::message::Message;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

pub struct DeadLetterQueue<T: Clone> {
    pub dlq: Arc<RwLock<VecDeque<Message<T>>>>, // Ensure thread-safe access.
    pub max_retries: usize,
    retries: RwLock<HashMap<u64, usize>>, // Make retries thread-safe as well.
}

impl<T: Clone> DeadLetterQueue<T> {
    pub fn new(max_retries: usize) -> Self {
        Self {
            dlq: Arc::new(RwLock::new(VecDeque::new())),
            max_retries,
            retries: RwLock::new(HashMap::new()),
        }
    }

    pub fn handle_failure(&self, message: Message<T>) {
        let mut retries_lock = self.retries.write().unwrap();
        let retry_count = retries_lock.entry(message.id).or_insert(0);
        *retry_count += 1;

        if *retry_count > self.max_retries {
            let mut dlq_lock = self.dlq.write().unwrap();
            dlq_lock.push_back(message);
        }
    }

    pub fn get_dead_letters(&self) -> Vec<Message<T>> {
        let dlq_lock = self.dlq.read().unwrap(); // Lock for reading.
        dlq_lock.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct CustomBody {
        content: String,
        priority: u8,
    }

    #[test]
    fn test_handle_failure_with_simple_body() {
        let dlq: DeadLetterQueue<String> = DeadLetterQueue::new(2);
        let message = Message {
            id: 1,
            body: "Test message".to_string(),
            attributes: None,
        };

        dlq.handle_failure(message.clone());
        assert!(dlq.get_dead_letters().is_empty()); // Not in DLQ on first failure.

        dlq.handle_failure(message.clone());
        assert!(dlq.get_dead_letters().contains(&message)); // Added to DLQ after max retries.
    }

    #[test]
    fn test_get_dead_letters_with_simple_body() {
        let dlq: DeadLetterQueue<String> = DeadLetterQueue::new(1);
        let message = Message {
            id: 1,
            body: "Test message".to_string(),
            attributes: None,
        };

        dlq.handle_failure(message.clone());
        assert_eq!(dlq.get_dead_letters(), vec![message]);
    }

    #[test]
    fn test_handle_failure_with_custom_body() {
        let dlq: DeadLetterQueue<CustomBody> = DeadLetterQueue::new(3);
        let message = Message {
            id: 1,
            body: CustomBody {
                content: "Important task".to_string(),
                priority: 5,
            },
            attributes: None,
        };

        dlq.handle_failure(message.clone());
        dlq.handle_failure(message.clone());
        assert!(dlq.get_dead_letters().is_empty()); // Not in DLQ yet.

        dlq.handle_failure(message.clone());
        assert!(dlq.get_dead_letters().contains(&message)); // Now in DLQ after max retries.
    }

    #[test]
    fn test_get_dead_letters_with_custom_body() {
        let dlq: DeadLetterQueue<CustomBody> = DeadLetterQueue::new(1);
        let message = Message {
            id: 1,
            body: CustomBody {
                content: "Low-priority task".to_string(),
                priority: 2,
            },
            attributes: None,
        };

        dlq.handle_failure(message.clone());
        let dead_letters = dlq.get_dead_letters();
        assert_eq!(dead_letters.len(), 1);
        assert_eq!(dead_letters[0], message);
    }
}
