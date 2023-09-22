macro_rules! jni_str {
    ($exp:expr, $str:expr) => {
        &$exp.new_string($str).unwrap()
    };
}

macro_rules! from_jni_str {
    ($exp:expr, $str:expr) => {
        $exp.get_string($str.borrow().l().unwrap().into())
    };
}

macro_rules! event_handler {
    ($handler:ident, $event:ident, $class_name:expr) => {
        pub struct $event<'a> {
            obj: crate::model::JavaObject<'a>
        }
        
        pub struct $handler<'a> {
            callback: Box<dyn FnMut(crate::model::bot::Bot, $event<'a>) -> ()>
        }
        
        impl<'a> From<crate::model::JavaObject<'a>> for $event<'a> {
            fn from(value: crate::model::JavaObject<'a>) -> Self {
                $event { obj: value }
            }
        }
        
        impl<'a> crate::event::EventHandler<'a> for $handler<'a> {
            type ET = $event<'a>;
        
            fn new<F: FnMut(crate::model::bot::Bot, Self::ET) -> () + 'static>(callback: F) -> Self {
                $handler { callback: Box::new(callback) }
            }
        
            fn class_name(&self) -> &'static str {
                $class_name
            }
        
            fn on_event(&mut self, bot: crate::model::bot::Bot, event_data: Self::ET) {
                (self.callback)(bot, event_data);
            }
        }
    };
}

#[macro_export]
macro_rules! use_events {
    () => {
        #[no_mangle]
        pub extern "system" fn Java_Bot_eventListener(mut env: jni::JNIEnv,
            _class: jni::objects::JClass,
            class_name: jni::objects::JString,
            ptr: i64,
            bot: jni::objects::JObject,
            event: jni::objects::JObject) {
                use mirai_jni_rs::event::EventHandler;
                use mirai_jni_rs::model::JavaObject;
                let jstr = env.get_string(&class_name).unwrap();
                let class_name = jstr.to_str().unwrap();
                match class_name {
                    mirai_jni_rs::classes::GROUP_MESSAGE_EVENT => {
                        let ptr = ptr as *mut mirai_jni_rs::event::group_message::GroupMessageHandler;
                        unsafe { ptr.as_mut().unwrap().on_event(JavaObject::new(&env, &bot).into(), JavaObject::new(&env, &event).into()) }
                    }
                    &_ => unreachable!()
                }
            }
    };
}