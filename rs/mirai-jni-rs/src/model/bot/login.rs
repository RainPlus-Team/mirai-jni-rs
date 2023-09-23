pub enum Protocol {
    AndroidPhone,
    AndroidPad,
    AndroidWatch,
    IPad,
    MacOS
}

pub enum BotAuthorization {
    Password(String),
    QRCode
}

impl From<String> for BotAuthorization {
    fn from(value: String) -> Self {
        BotAuthorization::Password(value)
    }
}