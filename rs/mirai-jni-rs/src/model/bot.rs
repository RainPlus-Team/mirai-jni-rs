use super::JavaObject;

pub mod login;
mod configuration;
mod action;
pub mod event;

pub struct Bot<'a> {
    obj: JavaObject<'a>
}

impl Bot<'_> {
    pub fn login(&mut self) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        env.call_method(&obj, "login", "()V", &[]).map(|_x| ())
    }
    pub fn is_online(&mut self) -> Result<bool, jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        env.call_method(&obj, "isOnline", "()Z", &[]).map(|x| x.z().unwrap())
    }
}

impl<'a> From<JavaObject<'a>> for Bot<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        Bot { obj: value }
    }
}