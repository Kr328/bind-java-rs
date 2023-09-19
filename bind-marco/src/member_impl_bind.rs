use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{member::Member, member_struct::StructForMember, modifier::ModifiersExt, types::Type};

pub struct ImplBindForMember<'a> {
    member: &'a Member,
}

impl<'a> ImplBindForMember<'a> {
    pub(crate) fn new(m: &'a Member) -> Self {
        ImplBindForMember { member: m }
    }
}

impl<'a> ToTokens for ImplBindForMember<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = StructForMember::new(self.member).field_name();

        fn method_signature(return_type: &Type, argument_types: impl Iterator<Item=Type>) -> String {
            let mut result = String::new();

            result.push('(');
            for t in argument_types {
                result.push_str(&t.to_signature());
            }
            result.push(')');

            result.push_str(&return_type.to_signature());

            result
        }

        let ts = match self.member {
            Member::Constructor { arguments, .. } => {
                let signature = method_signature(&Type::Void, arguments.iter().map(|a| a.type_name().to_type()));

                quote! {
                    #field_name: ::bind_java::find_method(ctx, class, "<init>", #signature)?
                }
            }
            Member::Method {
                name,
                return_type,
                arguments,
                modifiers,
                ..
            } => {
                let name = name.to_string();
                let signature = method_signature(&return_type.to_type(), arguments.iter().map(|a| a.type_name().to_type()));

                if modifiers.is_static() {
                    quote! {
                        #field_name: ::bind_java::find_static_method(ctx, class, #name, #signature)?
                    }
                } else {
                    quote! {
                        #field_name: ::bind_java::find_method(ctx, class, #name, #signature)?
                    }
                }
            }
            Member::Field {
                name,
                field_type,
                modifiers,
                ..
            } => {
                let name = name.to_string();
                let signature = field_type.to_type().to_signature();

                if modifiers.is_static() {
                    quote! {
                        #field_name: ::bind_java::find_static_field(ctx, class, #name, #signature)?
                    }
                } else {
                    quote! {
                        #field_name: ::bind_java::find_field(ctx, class, #name, #signature)?
                    }
                }
            }
        };

        tokens.extend(ts);
    }
}
