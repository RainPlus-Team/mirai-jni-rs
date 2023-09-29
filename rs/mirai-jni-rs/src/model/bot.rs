use crate::classes;

use super::JavaObject;

pub use super::bot_configuration::BotConfiguration;

pub mod login;
mod action;
pub mod event;

pub struct Bot<'a> {
    obj: JavaObject<'a>
}

impl<'a> Bot<'a> {
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