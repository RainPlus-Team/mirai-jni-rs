use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn bot_initialize(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    input.attrs.push(syn::parse_quote!(#[no_mangle]));
    input.sig.ident = syn::parse_str("Java_Bot_initialize").unwrap();
    input.sig.abi = syn::parse_str("extern \"system\"").unwrap();
    input.sig.inputs = syn::parse_quote!(mut env: jni::JNIEnv, class: jni::objects::JClass);

    TokenStream::from(quote!(#input))
}