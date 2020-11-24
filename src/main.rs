#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use serde::Serialize;
use rocket::*;
use std::fs::File;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize)]
struct Task { name : String, status  : bool }

fn main() {
    println!("Welcome to Rust Guessing Game !!");
    rocket::ignite()
        .mount("/", routes![index, hello, get_name, set_name,get_todos, get_todo_json])
        .launch();
}

#[get("/")] 
fn index() -> File {
    File::open("templates/index.html").expect("404, No file Found")
}
#[get("/hello")]
fn hello() -> String {
    format!("Hello, This is my first API Rust!")
}

#[get("/hello/<name>")]
fn get_name(name: String) -> String {
    format!("My Name is : {}", name)
}

#[post("/setname/<name>")]
fn set_name(name: String) -> String {
    format!("Data Posted successfully 🚀 ")
}

#[get("/todos")]
fn get_todos() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[get("/todojson")]
fn get_todo_json() -> Json<Task> {
    let a = Task{
        name : "Lets Learn Rust".to_string(),
        status : true
    };

    Json(a)
}
