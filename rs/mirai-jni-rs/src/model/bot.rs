use jni::{objects::JObject, JNIEnv};

use super::JavaObject;

pub mod login;
mod configuration;
mod action;
pub mod event;

pub struct Bot<'a> {
    env: JNIEnv<'a>,
    raw: JObject<'a>
}

impl Bot<'_> {
    pub fn login(&mut self) -> Result<(), jni::errors::Error> {
        self.env.call_method(&self.raw, "login", "()V", &[]).map(|_x| ())
    }
    pub fn is_online(&mut self) -> Result<bool, jni::errors::Error> {
        self.env.call_method(&self.raw, "isOnline", "()Z", &[]).map(|x| x.z().unwrap())
    }
}

impl<'a> From<JavaObject<'a>> for Bot<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        todo!()
    }
}

impl <'a>From<(JNIEnv<'a>, JObject<'a>)> for Bot<'a> {
    fn from(value: (JNIEnv<'a>, JObject<'a>)) -> Self {
        Bot {
            env: value.0,
            raw: value.1
        }
    }
}