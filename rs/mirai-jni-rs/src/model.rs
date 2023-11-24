use jni::{JNIEnv, objects::JObject};

pub struct JavaObject<'a> {
    env: JNIEnv<'a>,
    object: JObject<'a>
}

impl<'a> JavaObject<'a> {
    pub fn new(env: &JNIEnv<'a>, object: &JObject<'a>) -> Self {
        JavaObject { env: unsafe { env.unsafe_clone() }, object: unsafe { JObject::from_raw(**object) } }
    }
    pub fn r#use(&mut self) -> (&mut JNIEnv<'a>, &JObject<'a>) {
        (&mut self.env, &self.object)
    }
}

impl Clone for JavaObject<'_> {
    fn clone(&self) -> Self {
        Self { env: unsafe { self.env.unsafe_clone() }, object: unsafe { JObject::from_raw(*self.object) } }
    }
}

mod plugin_description;
pub use plugin_description::JvmPluginDescriptionBuilder;

pub mod env;
pub mod bot;
pub mod user;
pub mod member;