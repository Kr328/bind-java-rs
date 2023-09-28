use proc_macro2::Ident;
use syn::{
    LitStr,
    parenthesized,
    parse::{Parse, ParseStream},
    Token, token::Paren,
};

use crate::{
    repeat::{Repeat, Repeatable},
    types::ClassName,
};

pub struct Annotation {
    _at: Token![@],
    class_name: ClassName,
    _paren: Paren,
    value: LitStr,
}

impl Parse for Annotation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value_content;

        Ok(Annotation {
            _at: input.parse()?,
            class_name: input.parse()?,
            _paren: parenthesized!(value_content in input),
            value: value_content.parse()?,
        })
    }
}

impl Repeatable for Annotation {
    fn should_continue(input: ParseStream) -> bool {
        input.peek(Token![@])
    }
}

pub trait AnnotationsExt {
    fn alias(&self) -> Option<Ident>;
    fn class_name(&self) -> Option<String>;
}

impl AnnotationsExt for Repeat<Annotation> {
    fn alias(&self) -> Option<Ident> {
        self.values().iter().find_map(|a| {
            if a.class_name.to_class_name() == "Alias" {
                Some(Ident::new(&a.value.value(), a.value.span()))
            } else {
                None
            }
        })
    }

    fn class_name(&self) -> Option<String> {
        self.values().iter().find_map(|a| {
            if a.class_name.to_class_name() == "ClassName" {
                Some(a.value.value())
            } else {
                None
            }
        })
    }
}
