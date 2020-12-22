#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use futures::future;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
struct Ip {
    origin: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Book {
    name: String,
    imageurl: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Books {
    data: Vec<Book>,
}

use reqwest::Response;

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
    let data = get_data().await;
    match data {
        Err(_e) => println!("We have error"),
        Ok(d) => {
            let data = d.text().await;
            println!("{:?}", data);
        }
    }

    // let resp = get_json().await;
    // match resp {
    //     Err(e) => println!("Error {}", e),
    //     // Ok(d) => println!("Printing Name of Book {:?}", d.data[0].name),
    //     Ok(d) => println!("Printing Entire JSON : {:?}", d),
    // }
}

async fn get_data() -> Result<Response, Box<std::error::Error>> {
    let body = reqwest::get("https://www.google.com").await?;
    Ok(body)
}

async fn get_json() -> Result<Books, Box<dyn std::error::Error>> {
    let json: Books = reqwest::get("https://pokemon.proxy.beeceptor.com/books")
        .await?
        .json::<Books>()
        .await?;
    Ok(json)
}
