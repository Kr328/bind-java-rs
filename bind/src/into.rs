use std::ptr::null_mut;

use jni_sys::{
    jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jdouble, jdoubleArray, jfloat, jfloatArray, jint, jintArray,
    jlong, jlongArray, jobjectArray, jshort, jshortArray, jsize, jstring, JNI_FALSE, JNI_TRUE,
};

use crate::{call, with_pushed_frame, Context, Result};

pub trait IntoJava<T> {
    fn into_java(self, ctx: Context) -> Result<T>;
}

impl<T> IntoJava<T> for T {
    fn into_java(self, _: Context) -> Result<T> {
        Ok(self)
    }
}

impl IntoJava<jboolean> for bool {
    fn into_java(self, _: Context) -> Result<jboolean> {
        if self {
            Ok(JNI_TRUE)
        } else {
            Ok(JNI_FALSE)
        }
    }
}

impl IntoJava<jstring> for &str {
    fn into_java(self, ctx: Context) -> Result<jstring> {
        let utf16_chars = self.encode_utf16().collect::<Vec<_>>();

        Ok(unsafe { call!(ctx, NewString, utf16_chars.as_ptr().cast(), utf16_chars.len() as jsize) })
    }
}

macro_rules! array_impl {
    ($element_type:tt, $array_type:tt, $new_func:ident, $set_func:ident) => {
        impl IntoJava<$array_type> for &[$element_type] {
            fn into_java(self, ctx: Context) -> Result<$array_type> {
                let array = unsafe { call!(ctx, $new_func, self.len() as jsize) };

                unsafe { call!(ctx, $set_func, array, 0, self.len() as jsize, self.as_ptr()) };

                Ok(array)
            }
        }

        impl IntoJava<$array_type> for Vec<$element_type> {
            fn into_java(self, ctx: Context) -> Result<$array_type> {
                let array = unsafe { call!(ctx, $new_func, self.len() as jsize) };

                unsafe { call!(ctx, $set_func, array, 0, self.len() as jsize, self.as_ptr()) };

                Ok(array)
            }
        }
    };
}

array_impl!(jboolean, jbooleanArray, NewBooleanArray, SetBooleanArrayRegion);
array_impl!(jbyte, jbyteArray, NewByteArray, SetByteArrayRegion);
array_impl!(jchar, jcharArray, NewCharArray, SetCharArrayRegion);
array_impl!(jshort, jshortArray, NewShortArray, SetShortArrayRegion);
array_impl!(jint, jintArray, NewIntArray, SetIntArrayRegion);
array_impl!(jlong, jlongArray, NewLongArray, SetLongArrayRegion);
array_impl!(jfloat, jfloatArray, NewFloatArray, SetFloatArrayRegion);
array_impl!(jdouble, jdoubleArray, NewDoubleArray, SetDoubleArrayRegion);

impl IntoJava<jbooleanArray> for &[bool] {
    fn into_java(self, ctx: Context) -> Result<jbooleanArray> {
        let elements = self.iter().map(|&b| if b { JNI_TRUE } else { JNI_FALSE }).collect::<Vec<_>>();

        IntoJava::into_java(elements, ctx)
    }
}

impl IntoJava<jbooleanArray> for Vec<bool> {
    fn into_java(self, ctx: Context) -> Result<jbooleanArray> {
        IntoJava::into_java(&self[..], ctx)
    }
}

impl IntoJava<jobjectArray> for &[&str] {
    fn into_java(self, ctx: Context) -> Result<jobjectArray> {
        with_pushed_frame(ctx, self.len(), || {
            let array = unsafe {
                call!(
                    ctx,
                    NewObjectArray,
                    self.len() as jsize,
                    call!(ctx, FindClass, "java/lang/String\0".as_ptr().cast()),
                    null_mut()
                )
            };

            for (idx, &s) in self.iter().enumerate() {
                unsafe { call!(ctx, SetObjectArrayElement, array, idx as jsize, IntoJava::into_java(s, ctx)?) };
            }

            Ok(array)
        })
    }
}

impl IntoJava<jobjectArray> for &[String] {
    fn into_java(self, ctx: Context) -> Result<jobjectArray> {
        IntoJava::into_java(&self.iter().map(|s| s.as_str()).collect::<Vec<_>>()[..], ctx)
    }
}

impl IntoJava<jobjectArray> for Vec<String> {
    fn into_java(self, ctx: Context) -> Result<jobjectArray> {
        IntoJava::into_java(&self[..], ctx)
    }
}
