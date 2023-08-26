use crate::{classes, model::{bot::Bot, JavaObject}, model::member::Member};

use super::EventHandler;

pub struct GroupMessageEvent<'a> {
    obj: JavaObject<'a>
}

pub struct GroupMessageHandler<'a> {
    callback: Box<dyn FnMut(Bot, GroupMessageEvent<'a>) -> ()>
}

impl<'a> From<JavaObject<'a>> for GroupMessageEvent<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        GroupMessageEvent { obj: value }
    }
}

impl GroupMessageEvent<'_> {
    pub fn from_group(&mut self) -> i64 {
        let (env, obj) = self.obj.r#use();
        let group = env.call_method(&obj, "getGroup", format!("()L{};", classes::GROUP), &[]).unwrap();
        env.call_method(group.borrow().l().unwrap(), "getId", "()J", &[]).unwrap().j().unwrap()
    }
    pub fn sender<'a>(&'a mut self) -> Member {
        let (env, obj) = self.obj.r#use();
        let sender = env.call_method(&obj, "getSender", format!("()L{};", classes::MEMBER), &[]).unwrap();
        JavaObject::new(&env, &sender.l().unwrap()).into()
    }
    pub fn msg_str(&mut self) -> String {
        let (env, obj) = self.obj.r#use();
        let message = env.call_method(&obj, "getMessage", format!("()L{};", classes::MESSAGE_CHAIN), &[]).unwrap();
        let str = env.call_method(message.borrow().l().unwrap(), "contentToString", format!("()L{};", classes::STRING), &[]).unwrap();
        from_jni_str!(env, str).unwrap().into()
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