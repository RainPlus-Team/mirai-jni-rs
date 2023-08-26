use jni::{objects::JObject, JNIEnv};

use crate::{classes, model::bot::Bot, model::member::Member};

use super::EventHandler;

pub struct GroupMessageEvent<'a> {
    env: JNIEnv<'a>,
    raw: JObject<'a>
}

pub struct GroupMessageHandler<'a> {
    callback: Box<dyn FnMut(Bot, GroupMessageEvent<'a>) -> ()>
}

impl <'a>From<(JNIEnv<'a>, JObject<'a>)> for GroupMessageEvent<'a> {
    fn from(value: (JNIEnv<'a>, JObject<'a>)) -> Self {
        GroupMessageEvent {
            env: value.0,
            raw: value.1
        }
    }
}

impl GroupMessageEvent<'_> {
    pub fn from_group(&mut self) -> i64 {
        let group = self.env.call_method(&self.raw, "getGroup", format!("()L{};", classes::GROUP), &[]).unwrap();
        self.env.call_method(group.borrow().l().unwrap(), "getId", "()J", &[]).unwrap().j().unwrap()
    }
    pub fn sender<'a>(&'a self) -> Member {
        let mut env = unsafe { self.env.unsafe_clone() };
        let sender = env.call_method(&self.raw, "getSender", format!("()L{};", classes::MEMBER), &[]).unwrap();
        (env, sender.l().unwrap()).into()
    }
    pub fn msg_str(&mut self) -> String {
        let message = self.env.call_method(&self.raw, "getMessage", format!("()L{};", classes::MESSAGE_CHAIN), &[]).unwrap();
        let str = self.env.call_method(message.borrow().l().unwrap(), "contentToString", format!("()L{};", classes::STRING), &[]).unwrap();
        from_jni_str!(self.env, str).unwrap().into()
    }
}

impl<'a> EventHandler<'a> for GroupMessageHandler<'a> {
    type ET = GroupMessageEvent<'a>;

    fn new<F: FnMut(Bot, Self::ET) -> () + 'static>(callback: F) -> Self {
        GroupMessageHandler { callback: Box::new(callback) }
    }

    fn class_name(&self) -> &'static str {
        classes::GROUP_MESSAGE_EVENT
    }

    fn on_event(&mut self, bot: Bot, event_data: Self::ET) {
        (self.callback)(bot, event_data);
    }
}