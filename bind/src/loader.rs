use crate::{Class, Context, Result};

pub trait ClassLoader {
    fn load_class(&self, ctx: Context, name: &str) -> Result<Class>;
}
