use jni::{strings::JNIString, objects::JValueGen};

use crate::classes;

use super::{Bot, login::Protocol};

impl Bot<'_> {
    pub fn file_based_device_info<S: Into<JNIString>>(&mut self, file: S) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        let class = env.find_class(classes::BOT_CONF).unwrap();
        env.call_static_method(class, "setFileBasedDeviceInfo", format!("(L{};L{};)V", classes::BOT, classes::STRING), &[
            JValueGen::Object(&obj),
            JValueGen::Object(jni_str!(env, file))
        ]).map(|_x| ())
    }

    pub fn protocol(&mut self, prot: Protocol) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        let class = env.find_class(classes::BOT_CONF).unwrap();
        env.call_static_method(class, "setProtocol", format!("(L{};I)V", classes::BOT), &[
            JValueGen::Object(&obj),
            JValueGen::Int(prot as i32)
        ]).map(|_x| ())
    }
}