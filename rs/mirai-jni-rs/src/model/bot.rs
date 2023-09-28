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

    pub fn get_configuration() -> Result<BotConfiguration<'a>, jni::errors::Error> {
        todo!()
    }
    pub fn set_configuration() {
        todo!()
    }
}

impl<'a> From<JavaObject<'a>> for Bot<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        Bot { obj: value }
    }
}