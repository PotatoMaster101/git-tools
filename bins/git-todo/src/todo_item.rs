use serde::{Deserialize, Serialize};

/// Represents a TODO item.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    /// Item content.
    pub content: String,

    /// Whether the item is completed.
    pub completed: bool,
}

impl TodoItem {
    /// Creates a new `TodoItem`.
    pub fn new(content: String) -> Self {
        Self { content, completed: false }
    }

    /// Toggles the completion status of the item.
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = TodoItem::new("Test item".to_string());
        assert_eq!(item.content, "Test item");
        assert!(!item.completed);
    }

    #[test]
    fn test_toggle() {
        let mut item = TodoItem::new("Test item".to_string());
        item.toggle();
        assert!(item.completed);

        item.toggle();
        assert!(!item.completed);
    }
}
