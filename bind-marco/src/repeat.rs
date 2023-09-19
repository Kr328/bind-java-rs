use syn::parse::{Parse, ParseStream};

pub trait Repeatable {
    fn should_continue(input: ParseStream) -> bool;
}

pub struct Repeat<T> {
    values: Vec<T>,
}

impl<T> Repeat<T> {
    pub fn values(&self) -> &[T] {
        &self.values
    }
}

impl<T: Parse + Repeatable> Parse for Repeat<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut result = Vec::<T>::new();

        while !input.is_empty() && T::should_continue(input) {
            result.push(T::parse(input)?);
        }

        Ok(Repeat { values: result })
    }
}
