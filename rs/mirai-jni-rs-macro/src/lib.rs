use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn bot_initialize(_args: TokenStream, input: TokenStream) -> TokenStream {
    let event_listener: ItemFn = syn::parse_quote!(
    #[no_mangle]
    pub extern "system" fn Java_Bot_eventListener(mut env: jni::JNIEnv,
        _class: jni::objects::JClass,
        class_name: jni::objects::JString,
        ptr: i64,
        bot: jni::objects::JObject,
        event: jni::objects::JObject) {
            use mirai_jni_rs::bot::event::EventHandler;
            let jstr = env.get_string(&class_name).unwrap();
            let class_name = jstr.to_str().unwrap();
            match class_name {
                mirai_jni_rs::classes::GROUP_MESSAGE_EVENT => {
                    let ptr = ptr as *mut mirai_jni_rs::event::group_message::GroupMessageHandler;
                    unsafe { ptr.as_mut().unwrap().on_event((env.unsafe_clone(), bot).into(), (env, event).into()) }
                }
                &_ => unreachable!()
            }
        });

    let mut input = parse_macro_input!(input as ItemFn);

    input.attrs.push(syn::parse_quote!(#[no_mangle]));
    input.sig.ident = syn::parse_str(&format!("Java_{}", input.sig.ident)).unwrap();
    input.sig.abi = syn::parse_str("extern \"system\"").unwrap();
    input.sig.inputs = syn::parse_quote!(mut env: jni::JNIEnv, class: jni::objects::JClass);

    let mut init_fn = quote!(#input);
    init_fn.append_all(quote!(#event_listener).into_iter());

    TokenStream::from(init_fn)
}