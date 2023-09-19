use std::ptr::null_mut;

use jni_sys::{
    jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jdouble, jdoubleArray, jfloat, jfloatArray, jint, jintArray,
    jlong, jlongArray, jobject, jobjectArray, jshort, jshortArray, jsize, jstring, JNI_ABORT, JNI_FALSE,
};

use crate::{call, Context, Result};

pub trait FromJava<T>: Sized {
    unsafe fn from_java(value: T, ctx: Context) -> Result<Self>;
}

macro_rules! primitive_impl {
    ($typ:ty) => {
        impl FromJava<$typ> for $typ {
            unsafe fn from_java(value: $typ, _: Context) -> Result<Self> {
                Ok(value)
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

impl FromJava<jstring> for String {
    unsafe fn from_java(value: jstring, ctx: Context) -> Result<Self> {
        let length = unsafe { call!(ctx, GetStringLength, value) };
        let addr = unsafe { call!(ctx, GetStringChars, value, null_mut()) };

        let slice = unsafe { std::slice::from_raw_parts(addr, length as usize) };
        let result = String::from_utf16(slice);

        unsafe { call!(ctx, ReleaseStringChars, value, addr) };
        unsafe { call!(ctx, DeleteLocalRef, value) };

        Ok(result.unwrap())
    }
}

impl FromJava<jboolean> for bool {
    unsafe fn from_java(value: jboolean, _: Context) -> Result<Self> {
        Ok(value != JNI_FALSE)
    }
}

macro_rules! array_impl {
    ($element_type:tt, $array_type:tt, $get_elements_func:ident, $release_elements_func:ident) => {
        impl FromJava<$array_type> for Vec<$element_type> {
            unsafe fn from_java(value: $array_type, ctx: Context) -> Result<Self> {
                let length = unsafe { call!(ctx, GetArrayLength, value) };
                let addr = unsafe { call!(ctx, $get_elements_func, value, null_mut()) };

                let result = unsafe { std::slice::from_raw_parts(addr, length as usize) }.to_owned();

                unsafe { call!(ctx, $release_elements_func, value, addr, JNI_ABORT) };
                unsafe { call!(ctx, DeleteLocalRef, value) };

                Ok(result)
            }
        }
    };
}

array_impl!(jboolean, jbooleanArray, GetBooleanArrayElements, ReleaseBooleanArrayElements);
array_impl!(jbyte, jbyteArray, GetByteArrayElements, ReleaseByteArrayElements);
array_impl!(jchar, jcharArray, GetCharArrayElements, ReleaseCharArrayElements);
array_impl!(jshort, jshortArray, GetShortArrayElements, ReleaseShortArrayElements);
array_impl!(jint, jintArray, GetIntArrayElements, ReleaseIntArrayElements);
array_impl!(jlong, jlongArray, GetLongArrayElements, ReleaseLongArrayElements);
array_impl!(jfloat, jfloatArray, GetFloatArrayElements, ReleaseFloatArrayElements);
array_impl!(jdouble, jdoubleArray, GetDoubleArrayElements, ReleaseDoubleArrayElements);

impl FromJava<jbooleanArray> for Vec<bool> {
    unsafe fn from_java(value: jbooleanArray, ctx: Context) -> Result<Self> {
        let length = unsafe { call!(ctx, GetArrayLength, value) };
        let addr = unsafe { call!(ctx, GetBooleanArrayElements, value, null_mut()) };

        let result = unsafe { std::slice::from_raw_parts(addr, length as usize) }
            .iter()
            .map(|&b| b != JNI_FALSE)
            .collect::<Vec<_>>();

        unsafe { call!(ctx, ReleaseBooleanArrayElements, value, addr, JNI_ABORT) };
        unsafe { call!(ctx, DeleteLocalRef, value) };

        Ok(result)
    }
}

impl FromJava<jobjectArray> for Vec<String> {
    unsafe fn from_java(value: jobjectArray, ctx: Context) -> Result<Self> {
        let length = unsafe { call!(ctx, GetArrayLength, value) };

        let mut result = Vec::<String>::with_capacity(length as usize);
        for idx in 0..(length as usize) {
            let object = unsafe { call!(ctx, GetObjectArrayElement, value, idx as jsize) };

            match String::from_java(object, ctx) {
                Ok(v) => {
                    result.push(v);
                }
                Err(throwable) => {
                    unsafe { call!(ctx, DeleteLocalRef, value) };

                    return Err(throwable);
                }
            }
        }

        unsafe { call!(ctx, DeleteLocalRef, value) };

        Ok(result)
    }
}
