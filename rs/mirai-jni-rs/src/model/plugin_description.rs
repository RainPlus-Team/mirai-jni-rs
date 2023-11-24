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

    pub fn name<I>(&mut self, name: I) -> &Self
        where I: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "name", format!("(L{};)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
            JValueGen::Object(jni_str!(env, name))
        ]).unwrap();
        self
    }

    pub fn version<I>(&mut self, version: I) -> &Self
        where I: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "version", format!("(L{};)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
            JValueGen::Object(jni_str!(env, version))
        ]).unwrap();
        self
    }

    pub fn id<I>(&mut self, id: I) -> &Self
        where I: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "id", format!("(L{};)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
            JValueGen::Object(jni_str!(env, id))
        ]).unwrap();
        self
    }

    pub fn author<I>(&mut self, author: I) -> &Self
        where I: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "author", format!("(L{};)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
            JValueGen::Object(jni_str!(env, author))
        ]).unwrap();
        self
    }

    pub fn info<I>(&mut self, info: I) -> &Self
        where I: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "info", format!("(L{};)L{};", classes::STRING, classes::JVM_PLUGIN_DESCRIPTION_BUILDER), &[
            JValueGen::Object(jni_str!(env, info))
        ]).unwrap();
        self
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