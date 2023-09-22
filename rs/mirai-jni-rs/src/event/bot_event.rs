// TODO: Make events inheritable

use crate::{classes, model::{bot::Bot, JavaObject}};

pub struct BotEvent<'a> {
    obj: JavaObject<'a>
}

impl<'a> BotEvent<'a> {
    pub fn get_bot(&mut self) -> Bot<'a> {
        let (env, obj) = self.obj.r#use();
        let object = env.get_field(obj, "bot", format!("L{};", classes::BOT)).unwrap().l().unwrap();
        JavaObject::new(env, &object).into()
    }
}