use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
  
    let name= std::env::args().nth(1).expect("no url given");
    let url = format!("https://api.agify.io/?name={}", name);	
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    println!("Response body:\n{}", body);
    Ok(())
}
