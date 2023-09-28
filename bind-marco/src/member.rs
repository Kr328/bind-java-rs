use convert_case::{Case, Casing};
use proc_macro2::{Delimiter, Ident};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
    token::Paren,
};

use crate::{
    annotation::{Annotation, AnnotationsExt},
    argument::Argument,
    modifier::Modifier,
    repeat::Repeat,
    types::TypeName,
};

pub enum Member {
    Constructor {
        annotations: Repeat<Annotation>,
        name: Ident,
        _paren: Paren,
        arguments: Punctuated<Argument, Token![,]>,
    },
    Method {
        annotations: Repeat<Annotation>,
        modifiers: Repeat<Modifier>,
        return_type: TypeName,
        name: Ident,
        _paren: Paren,
        arguments: Punctuated<Argument, Token![,]>,
    },
    Field {
        annotations: Repeat<Annotation>,
        modifiers: Repeat<Modifier>,
        field_type: TypeName,
        name: Ident,
    },
}

impl Parse for Member {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let annotations: Repeat<Annotation> = input.parse()?;

        if let Some((_, cursor)) = input.cursor().ident() {
            if let Some(_) = cursor.group(Delimiter::Parenthesis) {
                let arguments_content;
                return Ok(Member::Constructor {
                    annotations,
                    name: input.parse()?,
                    _paren: parenthesized!(arguments_content in input),
                    arguments: Punctuated::parse_terminated(&arguments_content)?,
                });
            }
        }

        let modifiers: Repeat<Modifier> = input.parse()?;
        let type_name: TypeName = input.parse()?;
        let name: Ident = input.parse()?;
        if let Some(_) = input.cursor().group(Delimiter::Parenthesis) {
            let arguments_content;

            Ok(Member::Method {
                annotations,
                modifiers,
                return_type: type_name,
                name,
                _paren: parenthesized!(arguments_content in input),
                arguments: Punctuated::parse_terminated(&arguments_content)?,
            })
        } else {
            Ok(Member::Field {
                annotations,
                modifiers,
                field_type: type_name,
                name,
            })
        }
    }
}

impl Member {
    pub fn resolve_rust_name(&self) -> Ident {
        fn resolve(annotations: &Repeat<Annotation>, name: &Ident) -> Ident {
            if let Some(alias) = annotations.alias() {
                Ident::new(&alias.to_string(), name.span())
            } else {
                Ident::new(&name.to_string().to_case(Case::Snake), name.span())
            }
        }

        match self {
            Member::Constructor { annotations, name, .. } => resolve(annotations, &Ident::new("new", name.span())),
            Member::Method { annotations, name, .. } => resolve(annotations, name),
            Member::Field { annotations, name, .. } => resolve(annotations, name),
        }
    }
}
