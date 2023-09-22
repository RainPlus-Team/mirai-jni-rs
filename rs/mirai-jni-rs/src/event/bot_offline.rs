use jni::objects::JValueGen;

use crate::{classes, model::{bot::Bot, JavaObject}};

event_handler!(BotOfflineHandler, BotOfflineEvent, classes::BOT_OFFLINE_EVENT);

impl<'a> BotOfflineEvent<'a> {
    pub fn get_bot(&mut self) -> Bot<'a> {
        let (env, obj) = self.obj.r#use();
        let object = env.get_field(obj, "bot", format!("L{};", classes::BOT)).unwrap().l().unwrap();
        JavaObject::new(env, &object).into()
    }

    pub fn get_reconnect(&mut self) -> bool {
        let (env, obj) = self.obj.r#use();
        env.get_field(obj, "reconnect", "Z").unwrap().z().unwrap()
    }
    pub fn set_reconnect(&mut self, reconnect: bool) {
        let (env, obj) = self.obj.r#use();
        env.set_field(obj, "reconnect", "Z", JValueGen::Bool(reconnect as u8)).unwrap();
    }
}