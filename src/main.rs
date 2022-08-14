use clap::Parser;
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use serde_json::{self, value::Number};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author)]
struct CLI {
    /// Which date to get the picture from
    #[clap(value_parser)]
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    date: String,
    explanation: String,
    hdurl: String,
    media_type: String,
    service_version: String,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: Number,
    msg: String,
    service_version: String,
}

async fn parse_image(resp: Response) -> Result<(), Error> {

    let full_response = get(resp.url).await?;

    if full_response.status().is_success() {
        fs::write(resp.date + ".gif", full_response.bytes().await?).expect("File could not be created");
    }
    Ok(())
}

async fn make_request(body: &str, args: CLI) -> Result<(), Error> {
    let full_response = get(body.to_string() + &args.date).await?;

    if full_response.status().is_success() {
        let response = full_response.text().await?;

        let json: Response = serde_json::from_str(&response).unwrap();

        fs::write(format!("{}", json.date), format!("{:?}", json)).expect("Could not create text-file");

        parse_image(json).await?;
    } else {
        let response: ErrorResponse = serde_json::from_str(&full_response.text().await?).unwrap();

        println!("{:?}", response);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CLI::parse();

    let text = fs::read_to_string("api_key").unwrap();
    let body = format!("https://api.nasa.gov/planetary/apod?api_key={}&", text);

    make_request(&body, args).await?;

    Ok(())
}
