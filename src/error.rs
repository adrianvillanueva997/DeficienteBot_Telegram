use std::fmt;
use teloxide::RequestError;

#[derive(Debug)]
pub enum BotError {
    Teloxide(RequestError),
    Processing(String),
    Parse(String),
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::Teloxide(e) => write!(f, "Telegram API error: {e}"),
            BotError::Processing(e) => write!(f, "Processing error: {e}"),
            BotError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl std::error::Error for BotError {}
