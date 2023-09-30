use jni::{strings::JNIString, objects::JValueGen};

use crate::classes;

use super::Bot;

impl Bot<'_> {
    pub fn send_message<S: Into<JNIString>>(&mut self, group: i64, msg: S) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        let class = env.find_class(classes::BOT_ACTION).unwrap();
        env.call_static_method(class, "sendGroupMessage", format!("(L{};JL{};)V", classes::BOT, classes::STRING), &[
            JValueGen::Object(&obj),
            JValueGen::Long(group),
            JValueGen::Object(jni_str!(env, msg))
        ]).map(|_x| ())
    }
}