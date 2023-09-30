#[derive(Debug)]
pub enum BotAuthorization {
    Password(String),
    QRCode
}

impl From<String> for BotAuthorization {
    fn from(value: String) -> Self {
        BotAuthorization::Password(value)
    }
}