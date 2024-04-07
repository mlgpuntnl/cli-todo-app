
use std::{env, fs::File, io::Error as IoError};

use csv::StringRecord;

use super::TodoItem;

pub fn read_todo_items() -> Result<Vec<TodoItem>, IoError> {
    let file_path = get_file_path();
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(error) => return Result::Err(error),
    };
    let mut reader = csv::Reader::from_reader(file);
    let mut todo_items: Vec<TodoItem> = Vec::new();
    for result in reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(error) => return Result::Err(IoError::other(error)),
        };
        todo_items.push(create_todo_item(record));
    }
    Ok(todo_items)
}

pub fn write_todo_items(items: Vec<TodoItem>) -> Result<(), IoError> {
    let mut writer = csv::Writer::from_path(get_file_path())?;

    writer.write_record(&["id", "description", "priority", "done"])?;
    for item in items {
        writer.write_record(item.to_vector())?;
    }

    writer.flush()?;

    Ok(())
}

fn get_file_path() -> String {
    let result = env::var("TODOLIST_FILENAME");
    match result {
        Ok(filename) => filename,
        Err(error) => panic!("Failed loading enviorment {}", error),
    }
}

fn create_todo_item(record: StringRecord) -> TodoItem {
    if record.len() != 4 {
        panic!(
            "Incorect number of .csv columns. Need: 4, Got: {}",
            record.len()
        )
    }
    let done = record.get(3).unwrap().trim() == "1";
    return TodoItem {
        id: record.get(0).unwrap().trim().parse().unwrap(),
        description: record.get(1).unwrap().into(),
        priority: record.get(2).unwrap().trim().parse().unwrap(),
        done
    };
}