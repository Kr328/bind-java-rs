use crate::{Class, ClassLoader, Context, Result};

pub trait ClassBinding: Sized {
    unsafe fn bind(ctx: Context, class: Class) -> Result<Self>;
}

pub trait WithClass {
    const CLASS_NAME: &'static str;

    fn find_class(ctx: Context, loader: Option<&dyn ClassLoader>) -> Result<Class>;
}
