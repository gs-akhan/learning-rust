#[macro_use]
extern crate serde_derive;

use futures::future;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
struct Ip {
    origin: String,
}
#[tokio::main]
async fn main() {
    println!("Azhar is here");
    info!("New position: x: {}, y: {}", 1, 2);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.enter(|| {
        println!("in r.enter()");
        tokio::spawn(future::lazy(|_| {
            println!("WE are in the spwan");
        }));
    });
    get_data().await;
}

async fn get_data() {
    let body = reqwest::get("https://www.google.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", body);
}
