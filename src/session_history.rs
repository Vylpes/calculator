/// In-memory input history for a single interactive CLI session.
///
/// Entries are not persisted to disk. A new session always starts empty.
#[derive(Debug, Default, Clone)]
pub struct SessionHistory {
    entries: Vec<String>,
    /// Index while browsing history. `None` means the live (new) input line.
    cursor: Option<usize>,
}

impl SessionHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of saved history entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Saved entries in chronological order (oldest first).
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Whether `input` should be recorded in session history.
    ///
    /// Empty / whitespace-only lines and quit/exit commands are not stored.
    pub fn should_record(input: &str) -> bool {
        let trimmed = input.trim();
        !trimmed.is_empty() && trimmed != "quit" && trimmed != "exit"
    }

    /// Save a submitted line if it is meaningful. Resets the browse cursor.
    ///
    /// Returns `true` when the line was recorded.
    pub fn save(&mut self, input: &str) -> bool {
        if !Self::should_record(input) {
            self.cursor = None;
            return false;
        }

        self.entries.push(input.trim().to_string());
        self.cursor = None;
        true
    }

    /// Cycle backward through history (Up arrow).
    ///
    /// Returns the entry to show in the prompt, or `None` when history is empty
    /// or already at the oldest entry.
    pub fn previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let next_cursor = match self.cursor {
            None => self.entries.len() - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };
        self.cursor = Some(next_cursor);
        self.entries.get(next_cursor).map(String::as_str)
    }

    /// Cycle forward through history (Down arrow).
    ///
    /// Returns `Some("")` when moving past the newest entry onto a blank live
    /// line, the recalled entry while still browsing, or `None` when history is
    /// empty / already on the live line.
    pub fn next(&mut self) -> Option<&str> {
        let cursor = self.cursor?;

        if cursor + 1 >= self.entries.len() {
            self.cursor = None;
            return Some("");
        }

        self.cursor = Some(cursor + 1);
        self.entries.get(cursor + 1).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_records_non_empty_expressions() {
        let mut history = SessionHistory::new();
        assert!(history.save("2 + 3"));
        assert!(history.save("10 - 4"));
        assert_eq!(history.entries(), ["2 + 3", "10 - 4"]);
    }

    #[test]
    fn save_ignores_empty_and_whitespace() {
        let mut history = SessionHistory::new();
        assert!(!history.save(""));
        assert!(!history.save("   "));
        assert!(history.is_empty());
    }

    #[test]
    fn save_ignores_quit_and_exit() {
        let mut history = SessionHistory::new();
        assert!(!history.save("quit"));
        assert!(!history.save("exit"));
        assert!(history.is_empty());
    }

    #[test]
    fn save_trims_whitespace() {
        let mut history = SessionHistory::new();
        assert!(history.save("  2 + 3  "));
        assert_eq!(history.entries(), ["2 + 3"]);
    }

    #[test]
    fn previous_cycles_backward_from_newest() {
        let mut history = SessionHistory::new();
        history.save("1 + 1");
        history.save("2 + 2");
        history.save("3 + 3");

        assert_eq!(history.previous(), Some("3 + 3"));
        assert_eq!(history.previous(), Some("2 + 2"));
        assert_eq!(history.previous(), Some("1 + 1"));
        // Stays on oldest
        assert_eq!(history.previous(), Some("1 + 1"));
    }

    #[test]
    fn next_cycles_forward_to_blank_live_line() {
        let mut history = SessionHistory::new();
        history.save("1 + 1");
        history.save("2 + 2");
        history.save("3 + 3");

        history.previous();
        history.previous();
        history.previous();

        assert_eq!(history.next(), Some("2 + 2"));
        assert_eq!(history.next(), Some("3 + 3"));
        assert_eq!(history.next(), Some(""));
        assert_eq!(history.next(), None);
    }

    #[test]
    fn empty_history_up_and_down_do_nothing() {
        let mut history = SessionHistory::new();
        assert_eq!(history.previous(), None);
        assert_eq!(history.next(), None);
    }

    #[test]
    fn new_session_starts_empty() {
        let mut first = SessionHistory::new();
        first.save("7 - 1");
        assert!(!first.is_empty());

        let second = SessionHistory::new();
        assert!(second.is_empty());
        assert_eq!(second.entries(), [] as [&str; 0]);
    }

    #[test]
    fn up_recalls_latest_meaningful_entry_only() {
        // CLI skips calling save() for invalid expressions; history then
        // behaves as if only successful inputs were recorded.
        let mut history = SessionHistory::new();
        // "2 +" was invalid — not saved
        history.save("3 + 3");
        assert_eq!(history.previous(), Some("3 + 3"));
        assert_eq!(history.previous(), Some("3 + 3"));
        assert_eq!(history.len(), 1);
    }
}
