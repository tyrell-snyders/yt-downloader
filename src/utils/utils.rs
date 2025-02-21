use serde::{Deserialize, Serialize};


// structs
#[derive(Deserialize)]
pub struct Info {
    pub data: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub data: String,
}

#[derive(Serialize)]
pub struct VideoResponse {
    pub id: String,
    pub details: rustube::VideoDetails
}

//types
pub type Error = Box<dyn std::error::Error>;