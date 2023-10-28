use jni::objects::JValueGen;

use crate::classes;

use self::login::BotAuthorization;

use super::{JavaObject, env::MiraiEnv};

pub use super::bot_configuration::BotConfiguration;

pub mod login;
mod action;
pub mod event;

pub struct Bot<'a> {
    obj: JavaObject<'a>
}

impl<'a> Bot<'a> {
    pub fn new<T>(env: &'a MiraiEnv, qq: i64, auth: T) -> Self
        where T: Into<BotAuthorization>
    {
        let mut env = env.get_env();
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

    pub fn login(&mut self) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        env.call_method(&obj, "login", "()V", &[]).map(|_x| ())
    }
    pub fn is_online(&mut self) -> Result<bool, jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        env.call_method(&obj, "isOnline", "()Z", &[]).map(|x| x.z().unwrap())
    }

    simple_getter!(get_configuration, BotConfiguration<'a>, "configuration", l, format!("L{};", classes::BOT_CONFIGURATION), |env, _, result| {
        JavaObject::new(env, &result).into()
    });
}

impl<'a> From<JavaObject<'a>> for Bot<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        Bot { obj: value }
    }
}