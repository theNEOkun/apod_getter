mod response;

use crate::response::*;
use clap::Parser;
use regex::Regex;
use reqwest::{get, Error};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author)]
struct CLI {
    /// Which date to get the picture from
    #[clap(value_parser)]
    date: String,
}

impl CLI {
    pub fn get_date(&self) -> String {
        if Regex::new(r"\d{4}-\d{2}-\d{2}")
            .unwrap()
            .is_match(&self.date)
        {
            format!("date={}&", self.date)
        } else {
            format!("date={}&", "0000-00-00")
        }
    }
}

/**
 * Used to get the type of the image from the url
 * aka if the url has a .png, then it returns that
 *
 * ## Arguments
 *
 * * url - the url to split
 *
 * ## Return
 *
 * gets the last item after a dot
 */
fn get_type(url: &str) -> &str {
    let ulrs: Vec<&str> = url.split('.').collect();
    ulrs[ulrs.len() - 1]
}

/**
 * Used to get the image from the response
 *
 * ## Arguments
 *
 * * resp - the response from the server with all the information
 */
async fn parse_image(resp: Response) -> Result<(), Error> {
    let full_response = get(&resp.url).await?;

    if full_response.status().is_success() {
        let image_type = get_type(&resp.url);
        fs::write(
            format!("files/{}.{}", resp.date, image_type),
            full_response.bytes().await?,
        )
        .expect(&format!("File could not be created: {resp:?}"));
    }
    Ok(())
}

async fn make_request(body: &str, args: CLI) -> Result<(), Error> {
    let full_response = get(body.to_string() + &args.get_date()).await?;

    if full_response.status().is_success() {
        let response = full_response.text().await?;

        let json: Response = serde_json::from_str(&response).unwrap();

        fs::write(format!("files/{}", json.date), format!("{:?}", json))
            .expect("Could not create text-file");

        if json.media_type == "image" {
            parse_image(json).await?;
        } else {
            println!("Not an image");
        }
    } else {
        let response: ErrorResponse = serde_json::from_str(&full_response.text().await?).unwrap();

        println!("{}", response);
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
