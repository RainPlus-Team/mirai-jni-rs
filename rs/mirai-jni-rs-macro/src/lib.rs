use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn bot_initialize(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    input.attrs.push(syn::parse_quote!(#[no_mangle]));
    input.sig.ident = syn::parse_str("Java_Bot_initialize").unwrap();
    input.sig.abi = syn::parse_str("extern \"system\"").unwrap();
    input.sig.inputs = syn::parse_quote!(mut env: jni::JNIEnv, _: jni::objects::JClass);

    input.block.stmts.splice::<_, Vec<Stmt>>(0..0, syn::parse_quote!(
        let env = mirai_jni_rs::model::env::MiraiEnv::new(env);
    ));

    TokenStream::from(quote!(#input))
}