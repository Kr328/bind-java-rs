use std::ffi::CString;

use jni_sys::{jfieldID, jmethodID};

use crate::{call, invoke_with_throwable, Class, Context, Result};

pub fn find_class(ctx: Context, internal_name: &str) -> Result<Class> {
    let name = CString::new(internal_name).unwrap();

    unsafe { invoke_with_throwable(ctx, || call!(v1_1, ctx, FindClass, name.as_ptr())) }
}

pub fn find_method(ctx: Context, class: Class, name: &str, signature: &str) -> Result<jmethodID> {
    let name = CString::new(name).unwrap();
    let signature = CString::new(signature).unwrap();

    unsafe {
        invoke_with_throwable(ctx, || {
            call!(v1_1, ctx, GetMethodID, class, name.as_ptr(), signature.as_ptr())
        })
    }
}

pub fn find_static_method(ctx: Context, class: Class, name: &str, signature: &str) -> Result<jmethodID> {
    let name = CString::new(name).unwrap();
    let signature = CString::new(signature).unwrap();

    unsafe {
        invoke_with_throwable(ctx, || {
            call!(v1_1, ctx, GetStaticMethodID, class, name.as_ptr(), signature.as_ptr())
        })
    }
}

pub fn find_field(ctx: Context, class: Class, name: &str, signature: &str) -> Result<jfieldID> {
    let name = CString::new(name).unwrap();
    let signature = CString::new(signature).unwrap();

    unsafe { invoke_with_throwable(ctx, || call!(v1_1, ctx, GetFieldID, class, name.as_ptr(), signature.as_ptr())) }
}

pub fn find_static_field(ctx: Context, class: Class, name: &str, signature: &str) -> Result<jfieldID> {
    let name = CString::new(name).unwrap();
    let signature = CString::new(signature).unwrap();

    unsafe {
        invoke_with_throwable(ctx, || {
            call!(v1_1, ctx, GetStaticFieldID, class, name.as_ptr(), signature.as_ptr())
        })
    }
}
