
use reqwest;
use serde_json;

fn main() {
    match fetch() {
        Ok(()) => {},
        Err(err) => {
            eprintln!("{}",err);
        }
    }
}

#[tokio::main]
async fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let smeg_result = reqwest::get("https://smegadrive.ganymede.tv/api/random").await?.text().await?;
    let smeg_parsed: serde_json::Value = serde_json::from_str(&smeg_result).expect("Error reading JSON");


    println!("{:?}",smeg_parsed);

    Ok(())
}

// viuer: showing images
// reqwest, feed_rs
