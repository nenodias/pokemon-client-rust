use serde::{Serialize, Deserialize};
use reqwest::header::HeaderMap;

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    id: i32,
    name: String,
    height: i32,
    weight: i32,
    order: i32,
    is_default: bool
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..=151 {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        let resp = reqwest::Client::new()
            .get(format!("https://pokeapi.co/api/v2/pokemon/{}", i))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        let v: Pokemon = serde_json::from_str(&resp)?;
        println!("{:#?}", v);
    }
    Ok(())
}
