use reqwest::*;
#[macro_use]
extern crate serde_derive;
use reqwest::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    name: String,
    imageurl: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Books {
    data: Vec<Book>,
}

#[tokio::main]
async fn main() {
    println!("This is http file");
    let data = get_json().await;
    println!("{:?}", data);
}

async fn get_json() -> Result<Books> {
    // Fetching JSON and casting it to array of Books 
    let json: Books = reqwest::get("https://pokemon.proxy.beeceptor.com/books")
        .await?
        .json::<Books>()
        .await?;
    Ok(json)
}
