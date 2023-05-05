use reqwest;
use tokio;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u8,
    count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  
    let name= std::env::args().nth(1).expect("no url given");
    let url = format!("https://api.agify.io/?name={}", name);	
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    let data = serde_json::from_str::<Person>(&body)?;
    println!("Name: {:?}", data.name);
    println!("Age: {:?}", data.age);
    println!("Count: {:?}", data.count);
    Ok(())
}
