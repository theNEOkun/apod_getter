use clap::Parser;
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use serde_json::{self, value::Number};
use std::{env, fs};

#[derive(Parser, Debug)]
#[clap(author)]
struct CLI {
    #[clap(short, long)]
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

async fn make_request(body: &str, page: &str) -> Result<(), Error> {
    let full_response = get(body.to_string() + page).await?;

    if full_response.status().is_success() {
        let response = full_response.text().await?;

        let json: Response = serde_json::from_str(&response).unwrap();

        println!("{json:?}");
    } else {
        let response: ErrorResponse = serde_json::from_str(&full_response.text().await?).unwrap();

        println!("{:?}", response);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let body =
        "https://api.nasa.gov/planetary/apod?api_key=6wrs983OS2k0mSU7IWgSeuhoGt3aefu9xkZz5XMr&";

    make_request(body, &args[1]).await?;

    Ok(())
}
