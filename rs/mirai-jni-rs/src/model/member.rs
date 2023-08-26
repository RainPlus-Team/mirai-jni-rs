use jni::{objects::JObject, JNIEnv};

use crate::classes;

use super::{user::User, JavaObject};

pub struct Member<'a> {
    env: JNIEnv<'a>,
    raw: JObject<'a>,
    user: User<'a>
}

impl Member<'_> {
    pub fn id(&mut self) -> i64 {
        self.user.id()
    }
    pub fn nick(&mut self) -> String {
        self.user.nick()
    }
    pub fn name_card(&mut self) -> String {
        let str = self.env.call_method(&self.raw, "getNameCard", format!("()L{};", classes::STRING), &[]).unwrap();
        from_jni_str!(self.env, str).unwrap().into()
    }
}

impl<'a> From<JavaObject<'a>> for Member<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        todo!()
    }
}

impl <'a>From<(JNIEnv<'a>, JObject<'a>)> for Member<'a> {
    fn from(value: (JNIEnv<'a>, JObject<'a>)) -> Self {
        let user_env = unsafe { value.0.unsafe_clone() };
        Member {
            env: value.0,
            raw: unsafe { JObject::from_raw(value.1.clone()) },
            user: (user_env, value.1).into()
        }
    }
}
