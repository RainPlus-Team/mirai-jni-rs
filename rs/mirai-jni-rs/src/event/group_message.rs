use jni::{objects::JObject, JNIEnv};

use crate::{classes, bot::{Bot, event::EventHandler}, model::member::Member};

pub struct GroupMessageEvent<'a> {
    env: JNIEnv<'a>,
    raw: JObject<'a>
}

pub struct GroupMessageHandler<'a> {
    callback: fn(Bot, GroupMessageEvent<'a>) -> ()
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
        self.env.get_string(str.borrow().l().unwrap().into()).unwrap().into()
    }
}

impl <'a>EventHandler<'a, GroupMessageEvent<'a>> for GroupMessageHandler<'a> {
    fn new(callback: fn(Bot, GroupMessageEvent<'a>) -> ()) -> Self {
        GroupMessageHandler { callback }
    }

    fn class_name(&self) -> &'static str {
        classes::GROUP_MESSAGE_EVENT
    }

    fn on_event(&self, bot: Bot, event_data: GroupMessageEvent<'a>) {
        (self.callback)(bot, event_data);
    }
}