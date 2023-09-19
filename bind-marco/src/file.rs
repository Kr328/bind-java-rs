use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use crate::{class::Class, repeat::Repeat};

pub struct File {
    classes: Repeat<Class>,
}

impl Parse for File {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(File { classes: input.parse()? })
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for class in self.classes.values() {
            class.to_tokens(tokens)
        }
    }
}
