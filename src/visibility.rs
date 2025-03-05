use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct VisibilityControl {
    visibility_map: HashMap<u64, Instant>,
}

impl Default for VisibilityControl {
    fn default() -> Self {
        Self::new()
    }
}

impl VisibilityControl {
    pub fn new() -> Self {
        Self {
            visibility_map: HashMap::new(),
        }
    }

    pub fn is_visible(&self, message_id: u64) -> bool {
        if let Some(timeout) = self.visibility_map.get(&message_id) {
            Instant::now() >= *timeout
        } else {
            true
        }
    }

    pub fn set_visibility_timeout(&mut self, message_id: u64, timeout: Duration) {
        self.visibility_map
            .insert(message_id, Instant::now() + timeout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_set_visibility_timeout() {
        let mut visibility = VisibilityControl::new();
        let message_id = 1;

        visibility.set_visibility_timeout(message_id, Duration::from_secs(5));
        assert!(!visibility.is_visible(message_id)); // Not visible immediately.

        std::thread::sleep(Duration::from_secs(6));
        assert!(visibility.is_visible(message_id)); // Visible after timeout.
    }

    #[test]
    fn test_is_visible() {
        let visibility = VisibilityControl::new();
        assert!(visibility.is_visible(1));
    }
}
