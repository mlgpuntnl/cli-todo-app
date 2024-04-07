
use std::fmt::Display;

mod storage;
pub mod controller;

pub struct TodoItem {
    pub id: u32,
    pub description: String,
    pub priority: u16,
    pub done: bool,
}

impl TodoItem {
    pub fn new(description: String, priority: u16) -> Self {
        Self {
            id: 0,
            description,
            priority,
            done: false,
        }
    }

    pub fn to_vector(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.description.clone(),
            self.priority.to_string(),
            if self.done { String::from("1") } else { String::from("0") }
        ]
    }

}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ({}) {} | Done: {}",
            self.id, self.priority, self.description, self.done
        )
    }
}

impl Clone for TodoItem {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            description: self.description.clone(),
            priority: self.priority,
            done: self.done
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.id = source.id;
        self.description = source.description.clone();
        self.priority = source.priority;
        self.done = source.done;
    }
}
