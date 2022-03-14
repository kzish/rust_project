#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

//use serde::{Serialize, Deserialize};
//use serde_json::{Result, Value};
//use rocket::response::content;
use rocket::serde::json::Json;

#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;

#[cfg(test)]
use diesel::mysql::MySql;
use diesel::prelude::*;
use std::error::Error;
use std::time::SystemTime;



#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
struct Task {
    description: String,
    done: bool,
}

#[get("/getTodos")]
fn getTodos() -> Json<Vec<Task>> {

    let mut todoVec: Vec<Task> = Vec::new();
    let task1 = Task {
        description: "do something today".to_string(),
        done: false,
    };

    let task2 = Task {
        description: "did something yesterday".to_string(),
        done: true,
    };

    todoVec.push(task1);
    todoVec.push(task2);

    // // Serialize it to a JSON string.
    // let j = serde_json::to_string(&task1);
    // let k = serde_json::to_string(&task2);

    Json(todoVec)
    //Ok(j)

    
}

#[post("/createTodo/<description>")]
fn createTodo(description: String) -> String {

    let newTask = Task { 
        description: description.to_string(),
        done: false
    };
    //let newTask: Task = serde_json::from_str(&task).unwrap();
    
    format!("Hello, {}!", newTask.description)
}

#[post("/deleteTodo")]
fn deleteTodo() -> &'static str {
    "Hello, deleteTodo!"
}

#[post("/updateTodo")]
fn updateTodo() -> &'static str {
    "Hello, updateTodo"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![
        getTodos, 
        createTodo,
        deleteTodo,
        updateTodo
    ])
}
