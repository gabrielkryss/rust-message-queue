use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Message<T> {
    pub id: u64,
    pub body: T,
    pub attributes: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message {
            id: 1,
            body: "Test body".to_string(),
            attributes: Some(HashMap::new()),
        };
        assert_eq!(message.id, 1);
        assert_eq!(message.body, "Test body");
        assert!(message.attributes.is_some());
    }

    #[test]
    fn test_message_equality() {
        let message1 = Message {
            id: 1,
            body: "Test body".to_string(),
            attributes: None,
        };
        let message2 = Message {
            id: 1,
            body: "Test body".to_string(),
            attributes: None,
        };
        assert_eq!(message1, message2);
    }
}
