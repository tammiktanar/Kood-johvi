mod err;
use err::{ ParseErr, ReadErr };

use std::fs::File;
use std::io::Read;
pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new (title: String, tasks: Vec<Task>) -> TodoList {
        Self{title: title, tasks: tasks}
    }
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(err) => return Err(Box::new(ReadErr {child_err: Box::new(err)})),
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(err) => return Err(Box::new(ReadErr {child_err: Box::new(err)})),
        };

        let arr = match json::parse(&contents) {
            Ok(f) => f,
            Err(err) => return Err(Box::new(ParseErr::Malformed(Box::new(err)))),
        };

        let title = match arr["title"].as_str() {
            Some(f) => f.to_string(),
            None => return Err(Box::new(ParseErr::Empty)),
        };
        
        if !arr["tasks"].is_array() {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut task_list: Vec<Task> = Vec::new();

        for index in 0.. {
            if arr["tasks"][index].is_null() {
                break;
            }
            let task = Task{
                id: arr["tasks"][index]["id"].as_u32().unwrap(), 
                description: arr["tasks"][index]["description"].as_str().unwrap().to_string(),
                level: arr["tasks"][index]["level"].as_u32().unwrap(),
            };
            
            task_list.push(task);
        }

        if task_list.is_empty(){
            return Err(Box::new(ParseErr::Empty));
        }

        let res = TodoList::new(title.to_string(), task_list);
        Ok(res)
    }
}