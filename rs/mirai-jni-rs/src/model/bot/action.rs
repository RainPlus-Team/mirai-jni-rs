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

    pub fn send_image(&mut self, group: i64, img: &[i8]) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        let class = env.find_class(classes::HELPER).unwrap();
        let arr = env.new_byte_array(img.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(&arr, 0, img)?;
        let res = env.call_static_method(class, "bytesToExternalResource", format!("([B)L{};", classes::EXTERNAL_RESOURCE), &[
            JValueGen::Object(&arr)
        ]).unwrap().l().unwrap();

        let class = env.find_class(classes::BOT_ACTION).unwrap();
        env.call_static_method(class, "sendGroupImage", format!("(L{};JL{};)V", classes::BOT, classes::EXTERNAL_RESOURCE), &[
            JValueGen::Object(&obj),
            JValueGen::Long(group),
            JValueGen::Object(&res)
        ]).map(|_x| ())
    }
}