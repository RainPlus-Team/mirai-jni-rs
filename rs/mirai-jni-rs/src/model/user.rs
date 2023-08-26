use crate::classes;

use super::JavaObject;

pub struct User<'a> {
    obj: JavaObject<'a>
}

impl User<'_> {
    pub fn id(&mut self) -> i64 {
        let (env, obj) = self.obj.r#use();
        let id = env.call_method(&obj, "getId", "()J", &[]).unwrap();
        id.j().unwrap()
    }
    pub fn nick(&mut self) -> String { // TODO: extend it from UserOrBot
        let (env, obj) = self.obj.r#use();
        let str = env.call_method(&obj, "getNick", format!("()L{};", classes::STRING), &[]).unwrap();
        from_jni_str!(env, str).unwrap().into()
    }
}

impl<'a> From<JavaObject<'a>> for User<'a> {
    fn from(obj: JavaObject<'a>) -> Self {
        User { obj }
    }
}