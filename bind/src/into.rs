use std::ptr::null_mut;

use jni_sys::{
    jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jdouble, jdoubleArray, jfloat, jfloatArray, jint, jintArray,
    jlong, jlongArray, jobject, jobjectArray, jshort, jshortArray, jsize, jstring,
};

use crate::{call, with_pushed_frame, Context, Result};

pub trait IntoJava<T> {
    fn into_java(self, ctx: Context) -> Result<T>;
}

macro_rules! primitive_impl {
    ($typ:ty) => {
        impl IntoJava<$typ> for $typ {
            fn into_java(self, _: Context) -> Result<$typ> {
                Ok(self)
            }
        }
    };
}

primitive_impl!(jboolean);
primitive_impl!(jbyte);
primitive_impl!(jchar);
primitive_impl!(jshort);
primitive_impl!(jint);
primitive_impl!(jlong);
primitive_impl!(jfloat);
primitive_impl!(jdouble);
primitive_impl!(jobject);
primitive_impl!(());

impl IntoJava<jstring> for &str {
    fn into_java(self, ctx: Context) -> Result<jstring> {
        let utf16_chars = self.encode_utf16().collect::<Vec<_>>();

        Ok(unsafe { call!(v1_1, ctx, NewString, utf16_chars.as_ptr().cast(), utf16_chars.len() as jsize) })
    }
}

impl IntoJava<jstring> for String {
    fn into_java(self, ctx: Context) -> Result<jstring> {
        IntoJava::into_java(self.as_str(), ctx)
    }
}

impl IntoJava<jstring> for &String {
    fn into_java(self, ctx: Context) -> Result<jstring> {
        IntoJava::into_java(self.as_str(), ctx)
    }
}

impl<T: IntoJava<jobject>> IntoJava<jobject> for Option<T> {
    fn into_java(self, ctx: Context) -> Result<jobject> {
        let value = match self {
            Some(v) => v.into_java(ctx)?,
            None => null_mut(),
        };

        Ok(value)
    }
}

macro_rules! array_impl {
    ($element_type:tt, $array_type:tt, $new_func:ident, $set_func:ident) => {
        impl IntoJava<$array_type> for &[$element_type] {
            fn into_java(self, ctx: Context) -> Result<$array_type> {
                let array = unsafe { call!(v1_1, ctx, $new_func, self.len() as jsize) };

                unsafe { call!(v1_1, ctx, $set_func, array, 0, self.len() as jsize, self.as_ptr()) };

                Ok(array)
            }
        }

        impl IntoJava<$array_type> for Vec<$element_type> {
            fn into_java(self, ctx: Context) -> Result<$array_type> {
                let array = unsafe { call!(v1_1, ctx, $new_func, self.len() as jsize) };

                unsafe { call!(v1_1, ctx, $set_func, array, 0, self.len() as jsize, self.as_ptr()) };

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

impl IntoJava<jobjectArray> for &[&str] {
    fn into_java(self, ctx: Context) -> Result<jobjectArray> {
        with_pushed_frame(ctx, self.len(), || {
            let array = unsafe {
                call!(
                    v1_1,
                    ctx,
                    NewObjectArray,
                    self.len() as jsize,
                    call!(v1_1, ctx, FindClass, "java/lang/String\0".as_ptr().cast()),
                    null_mut()
                )
            };

            for (idx, &s) in self.iter().enumerate() {
                unsafe {
                    call!(
                        v1_1,
                        ctx,
                        SetObjectArrayElement,
                        array,
                        idx as jsize,
                        IntoJava::into_java(s, ctx)?
                    )
                };
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
