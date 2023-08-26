use jni::objects::JValueGen;

use crate::{classes, event::EventHandler};

use super::Bot;

impl<'a> Bot<'_> {
    pub fn register_event<E: EventHandler<'a>>(&mut self, event: E) -> Result<(), jni::errors::Error> {
        let (env, obj) = self.obj.r#use();
        
        let cls_name = event.class_name();
        let lang_class = env.find_class(classes::CLASS).unwrap();
        let event_class = env.call_static_method(lang_class, "forName", format!("(L{};)L{};", classes::STRING, classes::CLASS), &[
            JValueGen::Object(jni_str!(env, cls_name.replace("/", ".")))
        ]).unwrap();

        let ptr = Box::into_raw(Box::new(event));
        let class = env.find_class(classes::BOT_EVENT).unwrap();
        env.call_static_method(class, "registerEvent", format!("(L{};L{};J)V", classes::BOT, classes::CLASS), &[
            JValueGen::Object(&obj),
            event_class.borrow(),
            JValueGen::Long(ptr as i64)
        ]).map(|_x| ())
    }
}
