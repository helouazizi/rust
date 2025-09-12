mod err;
use json::JsonValue;
use crate::err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

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
    pub fn get_todo(path: &str) -> Result<TodoList, ReadErr> {
        let res = fs::read_to_string(path)
            .map_err(|e| ReadErr { child_err: Box::new(e) })?;
        // if res.trim().is_empty() {
        //     return Err(ReadErr { child_err: Box::new(ParseErr::Empty) });
        // }
        let parsed = json::parse(&res)
            .map_err(|e| ReadErr { child_err: Box::new(ParseErr::Malformed(Box::new(e))) })?;

        let title = parsed["title"].as_str().unwrap_or("").to_string();
        let mut tasks = Vec::new();
                  if parsed["tasks"].is_empty() {
            return Err(ReadErr { child_err: Box::new(ParseErr::Empty) });
        }
        for task in parsed["tasks"].members() {
          
            tasks.push(Task {
                id: task["id"].as_u32().unwrap_or(0),
                description: task["description"].as_str().unwrap_or("").to_string(),
                level: task["level"].as_u32().unwrap_or(0),
            });
        }

        Ok(TodoList { title, tasks })
    }
}