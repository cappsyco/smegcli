use reqwest;
use serde_json;
use viuer::{print_from_file, Config};

fn main() {
    match fetch() {
        Ok(meme) => {
            let mut frame = std::fs::File::create("smeg.jpg").unwrap();
            reqwest::blocking::get(meme.get_frameurl().as_str().unwrap())
                .unwrap()
                .copy_to(&mut frame)
                .unwrap();

            let conf = Config {
                absolute_offset: false,
                width: Some(50),
                ..Default::default()
            };
            print_from_file("smeg.jpg", &conf).expect("Image printing failed.");

            println!("");
            println!("{}", meme.get_series().as_str().unwrap());
            println!("{}", meme.get_episode().as_str().unwrap());
            println!("{}", meme.get_quote().as_str().unwrap());
            println!("{}", meme.get_frameurl().as_str().unwrap());
            println!("{}", meme.get_memelaburl().as_str().unwrap());
            println!("");

            println!("");
        }
        Err(err) => {
            eprintln!("Handled error: {}", err);
        }
    }
}

#[derive(Debug)]
struct Smegameme {
    series: serde_json::Value,
    episode: serde_json::Value,
    quote: serde_json::Value,
    frameurl: serde_json::Value,
    memelaburl: serde_json::Value,
}
impl Smegameme {
    pub fn get_series(&self) -> serde_json::Value {
        self.series.clone()
    }
    pub fn get_episode(&self) -> serde_json::Value {
        self.episode.clone()
    }
    pub fn get_quote(&self) -> serde_json::Value {
        self.quote.clone()
    }
    pub fn get_frameurl(&self) -> serde_json::Value {
        self.frameurl.clone()
    }
    pub fn get_memelaburl(&self) -> serde_json::Value {
        self.memelaburl.clone()
    }
}

#[tokio::main]
async fn fetch() -> Result<Smegameme, Box<dyn std::error::Error>> {
    let smeg_response = reqwest::get("https://smegadrive.ganymede.tv/api/random")
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{}", smeg_response["response"]["series"].as_str().unwrap());

    Ok(Smegameme {
        series: smeg_response["response"]["series"].clone(),
        episode: smeg_response["response"]["episode"].clone(),
        quote: smeg_response["response"]["quote"].clone(),
        frameurl: smeg_response["response"]["frame_url"].clone(),
        memelaburl: smeg_response["response"]["memelaburl"].clone(),
    })
}
