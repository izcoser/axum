use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePost {
    pub text: String,
}

#[derive(Serialize)]
pub struct Post {
    pub id: u64,
    pub text: String,
}