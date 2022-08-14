use serde::{Deserialize, Serialize};
use serde_json::{self, value::Number};
use std::fmt::Display;


/// Handles the response from the server, when it was a success
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    /// The date itself
    pub date: String,
    /// The explanation for the picture
    pub explanation: String,
    /// The HD url
    pub hdurl: Option<String>,
    /// What type the media is
    pub media_type: String,
    /// Who knows
    pub service_version: String,
    /// The title of the piece
    pub title: String,
    /// The URL to the picture
    pub url: String,
}

/// Handles the case for when an error is receieved
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    /// The code recieved
    pub code: Number,
    /// The message of why the error
    pub msg: String,
    pub service_version: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
code: {},
message: {},
", self.code, self.msg)
    }
}
