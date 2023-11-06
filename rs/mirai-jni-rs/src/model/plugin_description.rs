use jni::objects::JValueGen;

use crate::classes;

use super::{JavaObject, env::MiraiEnv};

pub struct ConsolePluginDescriptionBuilder<'a> {
    obj: JavaObject<'a>
}

impl<'a> ConsolePluginDescriptionBuilder<'a> {
    pub fn new<I, V, N>(env: &'a MiraiEnv, id: I, version: V, name: N) -> Self
        where I: AsRef<str>,
        V: AsRef<str>,
        N: AsRef<str> {
        let mut env = env.get_env();
        let class = env.find_class(classes::CONSOLE_PLUGIN_DESCRIPTION).unwrap();
        let obj = env.new_object(class, format!("(L{};L{};L{};)V", classes::STRING, classes::STRING, classes::STRING), &[
            JValueGen::Object(jni_str!(env, id)),
            JValueGen::Object(jni_str!(env, version)),
            JValueGen::Object(jni_str!(env, name))
        ]).unwrap();
        JavaObject::new(&env, &obj).into()
    }

    pub fn depends_on<I, V>(&mut self, id: I, version_requirement: Option<V>, is_optional: bool)
        where I: AsRef<str>,
            V: AsRef<str> {
        let (env, obj) = self.obj.r#use();
        if let Some(version_requirement) = version_requirement {
            env.call_method(obj, "dependsOn", format!("(L{};L{};Z)V", classes::STRING, classes::STRING), &[
                JValueGen::Object(jni_str!(env, id)),
                JValueGen::Object(jni_str!(env, version_requirement)),
                JValueGen::Bool(is_optional.into())
            ]).unwrap();
        } else {
            env.call_method(obj, "dependsOn", format!("(L{};Z)V", classes::STRING), &[
                JValueGen::Object(jni_str!(env, id)),
                JValueGen::Bool(is_optional.into())
            ]).unwrap();
        };
    }
}

impl<'a> From<JavaObject<'a>> for ConsolePluginDescriptionBuilder<'a> {
    fn from(obj: JavaObject<'a>) -> Self {
        ConsolePluginDescriptionBuilder { obj }
    }
}