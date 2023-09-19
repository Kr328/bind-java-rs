use std::ffi::CString;

use crate::{call, invoke_with_throwable, Class, Context, Result};

pub trait ClassLoader {
    fn load_class(&self, ctx: Context, name: &str) -> Result<Class>;
}

#[derive(Default)]
pub struct DefaultClassLoader {}

impl ClassLoader for DefaultClassLoader {
    fn load_class(&self, ctx: Context, name: &str) -> Result<Class> {
        let name = CString::new(name).unwrap();

        invoke_with_throwable(ctx, move || unsafe { call!(ctx, FindClass, name.as_ptr()) })
    }
}
