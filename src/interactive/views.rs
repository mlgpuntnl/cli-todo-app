use std::io;
use std::process;

use super::options::*;
use crate::util::terminal::*;
use crate::todo::controller as TodoController;
use crate::todo::TodoItem;

pub fn overview_page() {
    clear_screen();

    println!(
        "{}",
        ColoredText::from("Welcome to the TODO app!")
            .foreground_color(Color::Cyan)
            .text_style(TextStyle::Bold)
    );

    let todo_list = TodoController::get_todo_items();
    println!("Todolist length is now: {}", todo_list.len());

    for todo_item in &todo_list {
        println!("{}", todo_item)
    }
    print!("\n\n");

    let options = vec![
        OptionData::new(
            1,
            "Add new TODO item",
            ActionType::NoParameters(add_new_todo_item),
        ),
        OptionData::new(
            2,
            "Mark a TODO item as done",
            ActionType::NoParameters(mark_item_as_done),
        ),
        OptionData::new(3, "Exit TODO app", ActionType::NoParameters(exit)),
    ];

    handle_menu(&options);
}

fn add_new_todo_item() {
    clear_screen();
    println!(
        "{}",
        ColoredText::from("Create TODO item")
            .foreground_color(Color::Cyan)
            .text_style(TextStyle::Bold)
    );
    let mut item_description = String::new();
    println!(
        "{}",
        ColoredText::from("Please enter a description for your todo item:")
            .text_style(TextStyle::Bold)
    );
    io::stdin()
        .read_line(&mut item_description)
        .expect("Failed to read input");
    let item_description = String::from(item_description.trim());

    let item_priority: u16 = loop {
        let mut input_string = String::new();
        println!(
            "{}",
            ColoredText::from("Enter a TODO priority number. (Higher number = higher priority)")
                .text_style(TextStyle::Bold)
        );
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read input");
        match input_string.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                clear_line();
                continue;
            }
        };
    };
    let todo_item = TodoItem::new(item_description, item_priority);
    let options = vec![
        OptionData::new(
            1,
            "Save TODO item",
            ActionType::WithTodoItem(save_todo_item, todo_item),
        ),
        OptionData::new(
            2,
            "Discard changes and go back to overview",
            ActionType::NoParameters(overview_page),
        ),
    ];
    handle_menu(&options);
}

fn save_todo_item(item: &TodoItem) {
    clear_screen();
    TodoController::add_todo_item(item);
    println!("TODO item added");
    
    let options = vec![
        OptionData::new(1, "Back to overview", ActionType::NoParameters(overview_page))
    ];
    handle_menu(&options);
}

fn mark_item_as_done() {
    clear_screen();
    println!("Select one of the open TODOS to mark it as done");
    let open_todos = TodoController::get_open_todo_items();

    let mut options = vec![
        OptionData::new(0, "Back to overview", ActionType::NoParameters(overview_page))
    ];
    let mut i = 1;
    for todo in &open_todos {
        options.push(OptionData::new(
            i,
            todo.description.as_str(),
            ActionType::WithTodoItem(handle_todo_marked_as_done, todo.clone()))
        );
        i += 1;
    }

    handle_menu(&options);
}

fn handle_todo_marked_as_done(item: &TodoItem) {
    TodoController::mark_todo_item_as_done(item.id);
    println!("TODO item with id: {} marked as done", item.id);
    let options = vec![
        OptionData::new(1, "Back to overview", ActionType::NoParameters(overview_page))
    ];    
    handle_menu(&options)
}

fn exit() {
    println!("Goodbye!");
    process::exit(0);
}

fn handle_menu(options: &Vec<OptionData>) {
    println!("Please select one of the folowing actions:");
    for opt_row in options {
        if opt_row.number == 0 {
            println!(
                "{}",
                ColoredText::from(format!("{}: {}", opt_row.number, opt_row.title))
                    .text_style(TextStyle::Bold)
                    .foreground_color(Color::Red)
            );
        } else {
            println!(
                "{}",
                ColoredText::from(format!("{}: {}", opt_row.number, opt_row.title))
                    .text_style(TextStyle::Bold)
                    .foreground_color(Color::Green)
            );
        }
    }

    let chosen_option = loop {
        println!("Please type the number: ");
        let mut chosen_number = String::new();
        io::stdin()
            .read_line(&mut chosen_number)
            .expect("Failed to read input");
        let chosen_number: u16 = match chosen_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result = options
            .iter()
            .find(|opt_row| opt_row.number == chosen_number);
        match result {
            Some(i) => break i,
            None => continue,
        }
    };
    match &chosen_option.action {
        ActionType::NoParameters(f) => f(),
        ActionType::WithTodoItem(f, p) => f(p),
    }
}
