use reqwest::Client;
use crate::types::*;
use crate::error::BotError;

#[derive(Debug)]
pub struct TelegramBot {
    pub token: String,
    client: Client,
}

impl TelegramBot {
    pub fn new(token: &str) -> Self {
        TelegramBot {
            token: token.to_string(),
            client: Client::new(),
        }
    }

    
    pub async fn send_message(&self,chat_id: i64,text: &str,keyboard: Option<InlineKeyboardMarkup>,) -> Result<Message, BotError> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);

        let payload = SendMessageParams {
            chat_id,
            text: text.to_string(),
            parse_mode: None,
            reply_markup: keyboard,
        };

        let res = self.client.post(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let msg = res.json::<TelegramResponse<Message>>().await?;
        Ok(msg.result)
    }


    pub async fn get_updates(&self, offset: Option<i64>) -> Result<Vec<Update>, BotError> {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let mut query = vec![];
        if let Some(off) = offset {
            query.push(("offset", off.to_string()));
        }

        let res = self.client.get(&url)
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let updates = res.json::<TelegramResponse<Vec<Update>>>().await?;
        Ok(updates.result)
    }
}
