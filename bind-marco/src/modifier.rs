use syn::{
    parse::{Parse, ParseStream},
    token::{Final, Static},
    Token,
};

use crate::repeat::{Repeat, Repeatable};

pub enum Modifier {
    Static(Static),
    Final(Final),
}

impl Repeatable for Modifier {
    fn should_continue(input: ParseStream) -> bool {
        input.peek(Token![static]) || input.peek(Token![final])
    }
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![static]) {
            Ok(Modifier::Static(input.parse()?))
        } else if lookahead.peek(Token![final]) {
            Ok(Modifier::Final(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub trait ModifiersExt {
    fn is_static(&self) -> bool;
    fn is_final(&self) -> bool;
}

impl ModifiersExt for Repeat<Modifier> {
    fn is_static(&self) -> bool {
        self.values().iter().any(|m| matches!(m, Modifier::Static(_)))
    }

    fn is_final(&self) -> bool {
        self.values().iter().any(|m| matches!(m, Modifier::Final(_)))
    }
}
