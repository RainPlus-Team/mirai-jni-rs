use event::group_message::GroupMessageHandler;
use bot::event::EventHandler;
use jni::{JNIEnv, objects::{JClass, JString, JObject}};

pub use mirai_jni_rs_macro::bot_initialize;

#[macro_use]
mod macros;
pub mod model;
pub mod classes;
pub mod bot;
pub mod event;

#[no_mangle]
pub extern "system" fn Java_Bot_eventListener(mut env: JNIEnv,
    _class: JClass,
    class_name: JString,
    ptr: i64,
    bot: JObject,
    event: JObject) {
        let jstr = env.get_string(&class_name).unwrap();
        let class_name = jstr.to_str().unwrap();
        match class_name {
            classes::GROUP_MESSAGE_EVENT => {
                let ptr = ptr as *mut GroupMessageHandler;
                unsafe { ptr.as_ref().unwrap().on_event((env.unsafe_clone(), bot).into(), (env, event).into()) }
            }
            &_ => unreachable!()
        }
    }