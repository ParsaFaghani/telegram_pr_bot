use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SendMessageParams {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
}
