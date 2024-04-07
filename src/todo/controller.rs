
use std::cmp::Ordering;

use super::TodoItem;
use super::storage;

pub fn get_todo_items() -> Vec<TodoItem> {
    let result = storage::read_todo_items();
    match result {
        Ok(items) => items,
        Err(error) => panic!("Reading todo items failed with error: {:?}", error),
    }
}

pub fn get_open_todo_items() -> Vec<TodoItem> {
    let all_todo_items = get_todo_items();
    all_todo_items.into_iter().filter(|i| !i.done).collect()
}

pub fn add_todo_item(item: &TodoItem) {
    let mut cloned_item = item.clone();
    let mut all_todo_items: Vec<TodoItem> = get_todo_items();
    let mut highes_todo_id: u32 = 0;
    for todo_item in &all_todo_items {
        if todo_item.id > highes_todo_id {
            highes_todo_id = todo_item.id;
        }
    }
    cloned_item.id = highes_todo_id + 1;
    all_todo_items.push(cloned_item);

    all_todo_items.sort_by(|a, b| {
        if a.done > b.done {
            return Ordering::Less;
        } else if a.done < b.done {
            return Ordering::Greater;
        } else if a.priority > b.priority {
            return Ordering::Less;
        } else if a.priority < b.priority {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });

    let result = storage::write_todo_items(all_todo_items);
    match result {
        Err(error) => panic!("Adding TODO item failed with error: {:?}", error),
        Ok(()) => return
    };
}

pub fn mark_todo_item_as_done(todo_id: u32) {
    let mut all_todo_items = get_todo_items();

    for todo_item in all_todo_items.iter_mut() {
        if todo_item.id == todo_id {
            todo_item.done = true;
            break;
        }
    }
    let result = storage::write_todo_items(all_todo_items);
    match result {
        Err(error) => panic!("Adding TODO item failed with error: {:?}", error),
        Ok(()) => return
    };
}
