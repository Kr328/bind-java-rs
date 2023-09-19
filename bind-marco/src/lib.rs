use quote::ToTokens;

use crate::file::File;

mod annotation;
mod argument;
mod class;
mod file;
mod member;
mod member_impl;
mod member_impl_bind;
mod member_struct;
mod modifier;
mod repeat;
mod types;

#[proc_macro]
pub fn bind_java(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let file = syn::parse_macro_input!(input as File);

    file.into_token_stream().into()
}
