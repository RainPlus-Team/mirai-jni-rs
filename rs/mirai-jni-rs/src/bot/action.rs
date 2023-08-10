use jni::{strings::JNIString, objects::JValueGen};

use crate::classes;

use super::Bot;

impl Bot<'_> {
    pub fn send_message<S: Into<JNIString>>(&mut self, group: i64, msg: S) -> Result<(), jni::errors::Error> {
        let class = self.env.find_class(classes::BOT_ACTION).unwrap();
        self.env.call_static_method(class, "sendMessage", format!("(L{};JL{};)V", classes::BOT, classes::STRING), &[
            JValueGen::Object(&self.raw),
            JValueGen::Long(group),
            JValueGen::Object(jni_str!(self.env, msg))
        ]).map(|_x| ())
    }
}