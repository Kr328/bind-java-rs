use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::member::Member;

pub struct StructForMember<'a> {
    member: &'a Member,
}

impl<'a> StructForMember<'a> {
    pub fn new(member: &'a Member) -> Self {
        StructForMember { member }
    }

    pub fn field_name(&self) -> Ident {
        match &self.member {
            Member::Constructor { .. } => {
                format_ident!("c_{}", self.member.resolve_rust_name())
            }
            Member::Method { .. } => {
                format_ident!("m_{}", self.member.resolve_rust_name())
            }
            Member::Field { .. } => {
                format_ident!("f_{}", self.member.resolve_rust_name())
            }
        }
    }
}

impl<'a> ToTokens for StructForMember<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.field_name();

        let ts = match &self.member {
            Member::Constructor { .. } => {
                quote! { #name: ::jni_sys::jmethodID }
            }
            Member::Method { .. } => {
                quote! { #name: ::jni_sys::jmethodID }
            }
            Member::Field { .. } => {
                quote! { #name: ::jni_sys::jfieldID }
            }
        };

        tokens.extend(ts);
    }
}
