use telegram_pr_bot::bot::TelegramBot;
use telegram_pr_bot::types::{Update, Message};
use telegram_pr_bot::InlineKeyboardMarkup;
use telegram_pr_bot::InlineKeyboardButton;
use std::env;
use tokio;
use std;


#[tokio::main]
async fn main() {
    // گرفتن توکن از متغیر محیطی
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("توکن بات تنظیم نشده");
    let bot = TelegramBot::new(&token);
    let mut offset: Option<i64> = None;
    loop {
    // گرفتن آپدیت‌ها بدون offset
    match bot.get_updates(offset).await {
        Ok(updates) => {
            println!("updates: {:#?}", updates);
            for update in updates {
                offset = Some(update.update_id + 1);
                if let Some(callback_query) = update.callback_query {
                    let callback_query_id: &str = &callback_query.id;
                    println!("qbq id: {}", callback_query_id);
                    if let Err(e) = bot.answer_callback_query(&callback_query_id, Some("test"), Some(true)).await {
                        eprintln!("error: {}", e);
                    }
                    println!("an");

                }
                if let Some(message) = update.message {
                    let chat_id = message.chat.id;
                    
                    let text = message.text.unwrap_or_else(|| "👋 Hello!".to_string());
                    
                    let button1 = InlineKeyboardButton {
                        text: "channel 1".to_string(),
                        callback_data: Some("click 1".to_string()),
                        url: None,
                    };
                    let button2 = InlineKeyboardButton {
                        text: "channel 2".to_string(),
                        callback_data: Some("click 2".to_string()),
                        url: None,
                    };
                    let markup = InlineKeyboardMarkup {
                        inline_keyboard: vec![
                            vec![button1, button2],
                        ],
                    };
                    println!("📩 Received message: {}", text);

                    let reply = format!("You said: {}", text);
                    if let Err(e) = bot.send_message(chat_id, &reply, Some(markup)).await {
                        eprintln!("❌ Failed to send message: {}", e);
                    
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to get updates: {}", e);
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }   
}

