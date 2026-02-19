use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Add {
        username: String,
        password: String,
        service: String,
        secret: String,
    },
    List {
        username: String,
        password: String,
    },
    Fetch {
        username: String,
        password: String,
        service: String,
    },
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Secret { service: String, secret: String },
    Services { services: Vec<String> },
    Error { message: String },
}
