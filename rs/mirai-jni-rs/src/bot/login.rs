use jni::{JNIEnv, objects::JValueGen};

use crate::classes;

use super::Bot;

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

impl Bot<'_> {
    pub fn new<'a, T>(env: &mut JNIEnv<'a>, qq: i64, auth: T) -> Bot<'a>
        where T: Into<BotAuthorization>
    {
        let mut env = unsafe { env.unsafe_clone() };
        let auth: BotAuthorization = auth.into();

        let class = env.find_class(classes::BOT_AUTHORIZATION).unwrap();
        let j_auth =
        match auth {
            BotAuthorization::Password(password) => {
                env.call_static_method(class, "byPassword", format!("(L{};)L{};", classes::STRING, classes::BOT_AUTHORIZATION), &[
                    JValueGen::Object(jni_str!(env, password))
                ]).unwrap()
            },
            BotAuthorization::QRCode => {
                env.call_static_method(class, "byQRCode", format!("()L{};", classes::BOT_AUTHORIZATION), &[]).unwrap()
            }
        };

        let class = env.find_class(classes::BOT_FACTORY).unwrap();
        let inst = env.get_static_field(class, "INSTANCE", format!("L{}$INSTANCE;", classes::BOT_FACTORY)).unwrap().l().unwrap();
        let bot = env.call_method(inst, "newBot", format!("(JL{};)L{};", classes::BOT_AUTHORIZATION, classes::BOT), &[
            JValueGen::Long(qq),
            j_auth.borrow()
        ]).unwrap().l().unwrap();

        (env, bot).into()
    }
}