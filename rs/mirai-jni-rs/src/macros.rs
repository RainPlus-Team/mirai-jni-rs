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