use crate::todo::TodoItem;

pub enum ActionType {
    NoParameters(fn()),
    WithTodoItem(fn(&TodoItem), TodoItem),
}

pub struct OptionData<'a> {
    pub number: u16,
    pub title: &'a str,
    pub action: ActionType,
}

impl<'a> OptionData<'a> {
    pub fn new(number: u16, title: &'a str, action: ActionType) -> Self {
        Self {
            number,
            title,
            action,
        }
    }
}