use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Token,
};

use crate::{
    annotation::{Annotation, AnnotationsExt},
    member::Member,
    member_impl::ImplForMember,
    member_impl_bind::ImplBindForMember,
    member_struct::StructForMember,
    repeat::{Repeat, Repeatable},
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(class);
}

pub struct Class {
    annotations: Repeat<Annotation>,
    _class: kw::class,
    name: Ident,
    _brace: Brace,
    members: Punctuated<Member, Token![;]>,
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body_content;
        let class = Class {
            annotations: input.parse()?,
            _class: input.parse()?,
            name: input.parse()?,
            _brace: braced!(body_content in input),
            members: Punctuated::parse_terminated(&body_content)?,
        };

        for member in &class.members {
            if let Member::Constructor { name, .. } = member {
                if name != &class.name {
                    return Err(syn::Error::new(name.span(), "invalid constructor name."));
                }
            }
        }

        Ok(class)
    }
}

impl Repeatable for Class {
    fn should_continue(_: ParseStream) -> bool {
        true
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let class_name = self.annotations.class_name();
        let struct_fields = self.members.iter().map(|m| StructForMember::new(m));
        let struct_impls = self.members.iter().map(|m| ImplForMember::new(m));
        let struct_impl_bind = self.members.iter().map(|m| ImplBindForMember::new(m));

        tokens.extend(quote! {
            struct #name {
                #(#struct_fields),*
            }

            unsafe impl Sync for #name {}
            unsafe impl Send for #name {}

            impl #name {
                #(#struct_impls)*
            }

            impl ::bind_java::ClassBinding for #name {
                unsafe fn bind(ctx: ::bind_java::Context, class: ::bind_java::Class) -> ::bind_java::Result<Self> {
                    Ok(#name {
                        #(#struct_impl_bind),*
                    })
                }
            }
        });

        if let Some(class_name) = class_name {
            let internal_class_name = class_name.replace('.', "/");

            tokens.extend(quote! {
                impl ::bind_java::WithClass for #name {
                    const CLASS_NAME: &'static str = #class_name;

                    fn find_class(ctx: ::bind_java::Context, loader: ::std::option::Option<&dyn ::bind_java::ClassLoader>) -> ::bind_java::Result<::bind_java::Class> {
                        if let Some(loader) = loader {
                            loader.load_class(ctx, #class_name)
                        } else {
                            ::bind_java::find_class(ctx, #internal_class_name)
                        }
                    }
                }
            });
        }
    }
}
