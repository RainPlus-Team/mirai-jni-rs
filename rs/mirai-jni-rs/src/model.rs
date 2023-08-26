use jni::{JNIEnv, objects::JObject};

pub struct JavaObject<'a> {
    env: JNIEnv<'a>,
    object: JObject<'a>
}

impl<'a> JavaObject<'a> {
    pub fn new(env: JNIEnv<'a>, object: JObject<'a>) -> Self {
        JavaObject { env, object }
    }
    pub fn r#use(&self) -> (&JNIEnv, &JObject) {
        (&self.env, &self.object)
    }
}

pub mod bot;
pub mod user;
pub mod member;