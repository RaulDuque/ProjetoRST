//! Undo / redo of editor states (Equipment Studio, Engine 04). Pure module.
//!
//! Snapshot history: before an edit the editor records the state it is
//! about to change; undo swaps the current state with the last record (the
//! current one goes to the redo stack). A continuous edit (a mouse drag, a
//! held key) records once: [`History::begin`] opens a transaction and the
//! following [`History::record`] calls are ignored until [`History::end`].

/// One recorded state and what changed it.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry<T> {
    pub label: String,
    pub state: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct History<T> {
    undo: Vec<Entry<T>>,
    redo: Vec<Entry<T>>,
    limit: usize,
    open: bool,
}

impl<T: Clone> History<T> {
    pub fn new(limit: usize) -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            limit: limit.max(1),
            open: false,
        }
    }

    /// Records `before` (the state an edit is about to change). Inside a
    /// transaction only the first record counts.
    pub fn record(&mut self, label: &str, before: &T) {
        if self.open {
            return;
        }
        self.push(label, before);
    }

    /// Opens a transaction recording `before` once (a drag starts).
    pub fn begin(&mut self, label: &str, before: &T) {
        if !self.open {
            self.push(label, before);
            self.open = true;
        }
    }

    /// Closes the transaction (the drag ends).
    pub fn end(&mut self) {
        self.open = false;
    }

    fn push(&mut self, label: &str, before: &T) {
        self.undo.push(Entry {
            label: label.to_owned(),
            state: before.clone(),
        });
        if self.undo.len() > self.limit {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    /// The state to restore, given the current one (which becomes redoable).
    pub fn undo(&mut self, current: &T) -> Option<Entry<T>> {
        self.open = false;
        let e = self.undo.pop()?;
        self.redo.push(Entry {
            label: e.label.clone(),
            state: current.clone(),
        });
        Some(e)
    }

    /// The state to restore after an undo.
    pub fn redo(&mut self, current: &T) -> Option<Entry<T>> {
        self.open = false;
        let e = self.redo.pop()?;
        self.undo.push(Entry {
            label: e.label.clone(),
            state: current.clone(),
        });
        Some(e)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Label of the next undo.
    pub fn next_undo(&self) -> Option<&str> {
        self.undo.last().map(|e| e.label.as_str())
    }

    pub fn len(&self) -> usize {
        self.undo.len()
    }

    pub fn is_empty(&self) -> bool {
        self.undo.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_and_redo_walk_the_states() {
        let mut h = History::new(10);
        let mut state = 1;
        h.record("a", &state);
        state = 2;
        h.record("b", &state);
        state = 3;
        let back = h.undo(&state).unwrap_or_else(|| panic!("undo"));
        assert_eq!((back.label.as_str(), back.state), ("b", 2));
        state = back.state;
        let back = h.undo(&state).unwrap_or_else(|| panic!("undo"));
        state = back.state;
        assert_eq!(state, 1);
        assert!(h.undo(&state).is_none());
        let fwd = h.redo(&state).unwrap_or_else(|| panic!("redo"));
        assert_eq!(fwd.state, 2);
        state = fwd.state;
        let fwd = h.redo(&state).unwrap_or_else(|| panic!("redo"));
        assert_eq!(fwd.state, 3);
    }

    #[test]
    fn a_new_edit_clears_the_redo_stack() {
        let mut h = History::new(10);
        h.record("a", &1);
        assert!(h.undo(&2).is_some());
        assert!(h.can_redo());
        h.record("c", &1);
        assert!(!h.can_redo());
    }

    #[test]
    fn a_drag_is_one_operation() {
        let mut h = History::new(10);
        h.begin("drag", &0);
        for i in 1..100 {
            h.record("move", &i);
        }
        h.end();
        assert_eq!(h.len(), 1);
        assert_eq!(h.next_undo(), Some("drag"));
        assert_eq!(h.undo(&99).map(|e| e.state), Some(0));
    }

    #[test]
    fn the_history_is_bounded() {
        let mut h = History::new(3);
        for i in 0..10 {
            h.record("x", &i);
        }
        assert_eq!(h.len(), 3);
        assert_eq!(h.undo(&10).map(|e| e.state), Some(9));
    }
}
