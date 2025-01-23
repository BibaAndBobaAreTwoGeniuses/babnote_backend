use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct ResponseAuthOk {
    pub token: String
}

impl ResponseAuthOk {
    pub fn new(token: String) -> Self {
        Self { token: token.to_owned() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DownloadReq {
    pub filename: String,
}