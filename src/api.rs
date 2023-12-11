use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct API {
    req_client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoleContent {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptCompletionReq {
    model: String,
    messages: Vec<RoleContent>,
}

impl API {
    pub fn new() -> Self {
        API {
            req_client: Client::new(),
        }
    }

    // pub fn openai_chat_completion(&self, data: &ChatGptCompletionReq) ->
}
