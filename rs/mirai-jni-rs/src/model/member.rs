use jni::{objects::JObject, JNIEnv};

use crate::classes;

use super::{user::User, JavaObject};

pub struct Member<'a> {
    obj: JavaObject<'a>,
    user: User<'a>
}

impl Member<'_> {
    pub fn id(&mut self) -> i64 {
        self.user.id()
    }
    pub fn nick(&mut self) -> String {
        self.user.nick()
    }
    pub fn name_card(&mut self) -> String {
        let (env, obj) = self.obj.r#use();
        let str = env.call_method(&obj, "getNameCard", format!("()L{};", classes::STRING), &[]).unwrap();
        from_jni_str!(env, str).unwrap().into()
    }
}

impl<'a> From<JavaObject<'a>> for Member<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        Member { obj: value.clone(), user: value.into() }
    }
}

impl <'a>From<(JNIEnv<'a>, JObject<'a>)> for Member<'a> {
    fn from(value: (JNIEnv<'a>, JObject<'a>)) -> Self {
        Member {
            obj: JavaObject::new(&value.0, &value.1),
            user: JavaObject::new(&value.0, &value.1).into()
        }
    }
}
