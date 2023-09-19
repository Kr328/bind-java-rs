use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

use crate::types::{Type, TypeName};

pub struct Argument {
    type_name: TypeName,
    name: Ident,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Argument {
            type_name: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl Into<(Ident, Type)> for &Argument {
    fn into(self) -> (Ident, Type) {
        (self.name.clone(), self.type_name.to_type())
    }
}

impl Argument {
    pub fn new(type_name: TypeName, name: Ident) -> Self {
        Argument { type_name, name }
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn type_name(&self) -> &TypeName {
        &self.type_name
    }
}
