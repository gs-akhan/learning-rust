#![feature(proc_macro_hygiene, decl_macro)]
#[warn(unused_variables)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use log::debug;
use serde_json::Value as JValue;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

use log::*;
use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use std::fs::File;

#[derive(Serialize)]
struct Task {
    name: String,
    status: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    is_male: bool,
}

fn main() {
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    log::debug!("{}", "<<<<<<<<<<<<<<<<<<< Azhar is here >>>>>>>>");

    let json_string = r#"
        {
            "name" : "Azhar",
            "age" : 30,
            "is_male" : true
        }
    "#;

    let res = serde_json::from_str(json_string);

    if res.is_ok() {
        let p: JValue = res.unwrap();
        //let new_p: Person = res.unwrap();

        // println!("Name is {}", new_p.name);

        println!("Name is {:?}", p["name"].as_str().unwrap());
    } else {
        println!("Could not parse the json {:?}", res.unwrap());
    }

    println!("Welcome to Rust Guessing Game !!");
    rocket::ignite()
        .mount(
            "/",
            routes![index, hello, get_name, set_name, get_todos, get_todo_json],
        )
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

//
#[get("/todosjson")]
fn get_todo_json() -> Json<Vec<Task>> {
    let a = Task {
        name: "Lets Learn Rust".to_string(),
        status: true,
    };

    let b = Task {
        name: "Lets learn more Rust !!".to_string(),
        status: true,
    };

    Json(vec![a, b])
}
