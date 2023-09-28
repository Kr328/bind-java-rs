use syn::{
    parse::{Parse, ParseStream},
    token::{Final, Static},
    Token,
};

use crate::repeat::{Repeat, Repeatable};

pub mod kw {
    use syn::custom_keyword;

    custom_keyword!(native);
}

pub enum Modifier {
    Static(Static),
    Final(Final),
    Native(kw::native),
}

impl Repeatable for Modifier {
    fn should_continue(input: ParseStream) -> bool {
        input.peek(Token![static]) || input.peek(Token![final]) || input.peek(kw::native)
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![static]) {
            Ok(Modifier::Static(input.parse()?))
        } else if lookahead.peek(Token![final]) {
            Ok(Modifier::Final(input.parse()?))
        } else if lookahead.peek(kw::native) {
            Ok(Modifier::Native(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub trait ModifiersExt {
    fn is_static(&self) -> bool;
    fn is_final(&self) -> bool;
    fn is_native(&self) -> bool;
}

impl ModifiersExt for Repeat<Modifier> {
    fn is_static(&self) -> bool {
        self.values().iter().any(|m| matches!(m, Modifier::Static(_)))
    }

    fn is_final(&self) -> bool {
        self.values().iter().any(|m| matches!(m, Modifier::Final(_)))
    }

    fn is_native(&self) -> bool {
        self.values().iter().any(|m| matches!(m, Modifier::Native(_)))
    }
}
