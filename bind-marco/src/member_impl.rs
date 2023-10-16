use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::{
    member::Member,
    member_struct::StructForMember,
    modifier::{Modifier, ModifiersExt},
    repeat::Repeat,
    signature,
    types::Type,
};

#[derive(Copy, Clone)]
enum Target {
    This,
    Class,
}

impl Target {
    pub fn from_modifiers(modifiers: &Repeat<Modifier>) -> Target {
        if modifiers.is_static() {
            Target::Class
        } else {
            Target::This
        }
    }
}

fn build_invoke_func<'a>(
    name: &Ident,
    return_type: &Type,
    target: Target,
    func_name: &Ident,
    id_field_name: &Ident,
    arguments: &[(Ident, Type)],
) -> TokenStream {
    let (generic_list, return_type) = match return_type {
        Type::Void => (quote! {}, quote! { () }),
        typ => {
            let return_type = typ.render_jni_type();

            (quote! { <R: ::bind_java::FromJava<#return_type>> }, quote! { R })
        }
    };
    let (target_name, target_type) = match target {
        Target::This => (Ident::new("this", Span::call_site()), quote! { ::bind_java::Object }),
        Target::Class => (Ident::new("class", Span::call_site()), quote! { ::bind_java::Class }),
    };
    let args_names = arguments
        .iter()
        .map(|a| Ident::new(&a.0.to_string().to_case(Case::Snake), a.0.span()))
        .collect::<Vec<_>>();
    let args_types = arguments.iter().map(|a| a.1.render_jni_type()).collect::<Vec<_>>();

    quote! {
        pub unsafe fn #name #generic_list (
            &self,
            ctx: ::bind_java::Context,
            #target_name: #target_type,
            #(#args_names: impl ::bind_java::IntoJava<#args_types>),*
        ) -> ::bind_java::Result<#return_type> {
            ::bind_java::_invoke!(
                #return_type,
                ctx,
                #func_name,
                #target_name,
                self.#id_field_name,
                #(#args_names),*
            )
        }
    }
}

fn build_register_func(
    name: &Ident,
    return_type: &Type,
    target: Target,
    method_name: &str,
    arguments: &[(Ident, Type)],
) -> TokenStream {
    let signature = signature::method_signature(return_type, arguments.iter().map(|t| t.1.clone()));

    let args_types = arguments.iter().map(|t| t.1.render_jni_type()).collect::<Vec<_>>();
    let return_type = return_type.render_jni_type();
    let target_type = match target {
        Target::This => quote! { ::bind_java::Object },
        Target::Class => quote! { ::bind_java::Class },
    };

    quote! {
        pub unsafe fn #name (
            ctx: ::bind_java::Context,
            class: ::bind_java::Class,
            handler: extern "system" fn(
                ::bind_java::Context,
                #target_type,
                #(#args_types),*
            ) -> #return_type,
        ) -> ::bind_java::Result<()> {
            ::bind_java::register_native_method(
                ctx,
                class,
                #method_name,
                #signature,
                handler as *const (),
            )
        }
    }
}

fn resolve_call_func_name(return_type: &Type, modifiers: &Repeat<Modifier>) -> Ident {
    if modifiers.is_static() {
        return_type.to_call_static_method_name()
    } else {
        return_type.to_call_method_name()
    }
}

fn resolve_get_field_name(return_type: &Type, modifiers: &Repeat<Modifier>) -> Ident {
    if modifiers.is_static() {
        return_type.to_get_static_field_method_name()
    } else {
        return_type.to_get_field_method_name()
    }
}

fn resolve_set_field_name(return_type: &Type, modifiers: &Repeat<Modifier>) -> Ident {
    if modifiers.is_static() {
        return_type.to_set_static_field_method_name()
    } else {
        return_type.to_set_field_method_name()
    }
}

pub struct ImplForMember<'a> {
    member: &'a Member,
}

impl<'a> ImplForMember<'a> {
    pub fn new(member: &'a Member) -> Self {
        ImplForMember { member }
    }
}

impl<'a> ToTokens for ImplForMember<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rs_name = self.member.resolve_rust_name();
        let field_name = StructForMember::new(&self.member).field_name();

        match &self.member {
            Member::Constructor {
                annotations: _annotations,
                name: _name,
                _paren,
                arguments,
            } => {
                tokens.extend(build_invoke_func(
                    &rs_name,
                    &Type::Object("java/lang/Object".to_owned()),
                    Target::Class,
                    &Ident::new("NewObject", Span::call_site()),
                    &field_name,
                    &arguments.iter().map(|a| a.into()).collect::<Vec<_>>(),
                ));
            }
            Member::Method {
                annotations: _annotations,
                modifiers,
                return_type,
                name,
                _paren,
                arguments,
            } => {
                let return_type = return_type.to_type();
                let target = Target::from_modifiers(modifiers);
                let arguments = arguments.iter().map(|a| a.into()).collect::<Vec<_>>();

                tokens.extend(build_invoke_func(
                    &rs_name,
                    &return_type,
                    target,
                    &resolve_call_func_name(&return_type, modifiers),
                    &field_name,
                    &arguments,
                ));

                if modifiers.is_native() {
                    tokens.extend(build_register_func(
                        &format_ident!("register_{}", rs_name),
                        &return_type,
                        target,
                        &name.to_string(),
                        &arguments,
                    ))
                }
            }
            Member::Field {
                annotations: _annotations,
                modifiers,
                field_type,
                name: _name,
            } => {
                let field_type = field_type.to_type();

                tokens.extend(build_invoke_func(
                    &format_ident!("get_{}", rs_name),
                    &field_type,
                    Target::from_modifiers(modifiers),
                    &resolve_get_field_name(&field_type, modifiers),
                    &field_name,
                    &[],
                ));

                if !modifiers.is_final() {
                    tokens.extend(build_invoke_func(
                        &format_ident!("set_{}", rs_name),
                        &Type::Void,
                        Target::from_modifiers(modifiers),
                        &resolve_set_field_name(&field_type, modifiers),
                        &field_name,
                        &[(Ident::new("value", rs_name.span()), field_type)],
                    ));
                }
            }
        }
    }
}
