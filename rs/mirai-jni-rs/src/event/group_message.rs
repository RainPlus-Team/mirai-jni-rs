use crate::{classes, model::JavaObject, model::member::Member};

event_handler!(GroupMessageHandler, GroupMessageEvent, classes::GROUP_MESSAGE_EVENT);

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