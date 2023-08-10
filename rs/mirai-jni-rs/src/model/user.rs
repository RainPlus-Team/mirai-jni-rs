use jni::{JNIEnv, objects::JObject};

use crate::classes;

pub struct User<'a> {
    env: JNIEnv<'a>,
    raw: JObject<'a>
}

impl User<'_> {
    pub fn id(&mut self) -> i64 {
        let id = self.env.call_method(&self.raw, "getId", "()J", &[]).unwrap();
        id.j().unwrap()
    }
    pub fn nick(&mut self) -> String { // TODO: extend it from UserOrBot
        let str = self.env.call_method(&self.raw, "getNick", format!("()L{};", classes::STRING), &[]).unwrap();
        self.env.get_string(str.borrow().l().unwrap().into()).unwrap().into()
    }
}

impl <'a>From<(JNIEnv<'a>, JObject<'a>)> for User<'a> {
    fn from(value: (JNIEnv<'a>, JObject<'a>)) -> Self {
        User {
            env: value.0,
            raw: value.1
        }
    }
}