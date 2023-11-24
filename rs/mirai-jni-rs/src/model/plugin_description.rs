use jni::objects::JValueGen;

use crate::classes;

use super::{JavaObject, env::MiraiEnv};

pub struct JvmPluginDescriptionBuilder<'a> {
    obj: JavaObject<'a>
}

impl<'a> JvmPluginDescriptionBuilder<'a> {
    pub fn new<I, V, N>(env: &'a MiraiEnv, id: I, version: V) -> Self
        where I: AsRef<str>,
        V: AsRef<str>,
        N: AsRef<str> {
        let mut env = env.get_env();
        let class = env.find_class(classes::JVM_PLUGIN_DESCRIPTION_BUILDER).unwrap();
        let obj = env.new_object(class, format!("(L{};L{};)V", classes::STRING, classes::STRING), &[
            JValueGen::Object(jni_str!(env, id)),
            JValueGen::Object(jni_str!(env, version))
        ]).unwrap();
        JavaObject::new(&env, &obj).into()
    }

    pub fn depends_on<I, V>(&mut self, id: I, version_requirement: Option<V>, is_optional: bool)
        where I: AsRef<str>,
            V: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        if let Some(version_requirement) = version_requirement {
            env.call_method(obj, "dependsOn", format!("(L{};L{};Z)L{};", classes::STRING, classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
                JValueGen::Object(jni_str!(env, id)),
                JValueGen::Object(jni_str!(env, version_requirement)),
                JValueGen::Bool(is_optional.into())
            ]).unwrap();
        } else {
            env.call_method(obj, "dependsOn", format!("(L{};Z)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
                JValueGen::Object(jni_str!(env, id)),
                JValueGen::Bool(is_optional.into())
            ]).unwrap();
        };
    }
}

impl<'a> From<JavaObject<'a>> for JvmPluginDescriptionBuilder<'a> {
    fn from(obj: JavaObject<'a>) -> Self {
        JvmPluginDescriptionBuilder { obj }
    }
}