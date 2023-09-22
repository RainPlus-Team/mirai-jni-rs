use crate::{classes, model::{bot::Bot, JavaObject}};

event_handler!(BotOnlineHandler, BotOnlineEvent, classes::BOT_ONLINE_EVENT);

impl<'a> BotOnlineEvent<'a> {
    pub fn get_bot(&mut self) -> Bot<'a> {
        let (env, obj) = self.obj.r#use();
        let object = env.get_field(obj, "bot", format!("L{};", classes::BOT)).unwrap().l().unwrap();
        JavaObject::new(env, &object).into()
    }
}