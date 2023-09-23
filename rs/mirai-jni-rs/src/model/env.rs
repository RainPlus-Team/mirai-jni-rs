use jni::{JNIEnv, objects::JValueGen};

use crate::classes;

use super::{bot::{Bot, login::BotAuthorization}, JavaObject};

pub struct MiraiEnv<'a> {
    env: JNIEnv<'a>
}

impl<'a> MiraiEnv<'a> {
    pub fn new(env: JNIEnv<'a>) -> Self {
        MiraiEnv { env }
    }

    pub fn create_bot<T>(&self, qq: i64, auth: T) -> Bot<'a>
        where T: Into<BotAuthorization>
    {
        let mut env = unsafe { self.env.unsafe_clone() };
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

        JavaObject::new(&env, &bot).into()
    }
}