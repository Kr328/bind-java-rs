use std::ptr::null_mut;

use jni_sys::{jboolean, jbyte, jchar, jdouble, jfieldID, jfloat, jint, jlong, jmethodID, jobject, jshort, jsize};

use crate::{call, Context, Result};

pub trait AsMutObject {
    fn as_mut_object(&mut self) -> Option<&mut jobject>;
}

impl AsMutObject for jobject {
    fn as_mut_object(&mut self) -> Option<&mut jobject> {
        Some(self)
    }
}

impl<T: AsMutObject> AsMutObject for Result<T> {
    fn as_mut_object(&mut self) -> Option<&mut jobject> {
        match self {
            Ok(obj) => obj.as_mut_object(),
            Err(th) => Some(th),
        }
    }
}

macro_rules! none_mut_object {
    ($type_name:tt) => {
        impl AsMutObject for $type_name {
            fn as_mut_object(&mut self) -> Option<&mut jobject> {
                None
            }
        }
    };
}

none_mut_object!(jboolean);
none_mut_object!(jbyte);
none_mut_object!(jchar);
none_mut_object!(jshort);
none_mut_object!(jint);
none_mut_object!(jlong);
none_mut_object!(jfloat);
none_mut_object!(jdouble);
none_mut_object!(jmethodID);
none_mut_object!(jfieldID);
none_mut_object!(());

pub fn with_pushed_frame<R: AsMutObject, F: FnOnce() -> R>(ctx: Context, min_size: usize, f: F) -> R {
    unsafe { call!(ctx, PushLocalFrame, min_size as jsize) };

    let mut r = f();

    if let Some(object) = r.as_mut_object() {
        *object = unsafe { call!(ctx, PopLocalFrame, *object) };
    } else {
        unsafe { call!(ctx, PopLocalFrame, null_mut()) };
    }

    r
}
