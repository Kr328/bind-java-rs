use std::ffi::CString;

use jni_sys::{jfieldID, jmethodID, JNINativeMethod};

use crate::{call, Class, Context, invoke_with_throwable, Result};

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

pub fn register_native_method(ctx: Context, class: Class, name: &str, signature: &str, func: *const ()) -> Result<()> {
    unsafe {
        let name = CString::new(name).unwrap();
        let signature = CString::new(signature).unwrap();

        let m = JNINativeMethod {
            name: name.as_ptr().cast_mut(),
            signature: signature.as_ptr().cast_mut(),
            fnPtr: func.cast_mut().cast(),
        };

        invoke_with_throwable(ctx, || call!(v1_1, ctx, RegisterNatives, class, &m, 1))?;
    }

    Ok(())
}
