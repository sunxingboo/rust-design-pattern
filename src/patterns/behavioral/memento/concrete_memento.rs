use std::rc::Rc;
use super::memento::Memento;
use super::originator::Originator;

/// 编辑器快照。
pub struct Snapshot {
    text: String,
}

impl Memento for Snapshot {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Snapshot {
    pub fn new(text: String) -> Self {
        Snapshot {
            text: text.to_string(),
        }
    }
}