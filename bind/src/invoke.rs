use jni_sys::{jboolean, jbyte, jchar, jdouble, jfieldID, jfloat, jint, jlong, jmethodID, jobject, jshort, jvalue};
use paste::paste;
use std::ptr::null_mut;

use crate::{call, Class, Context, FromJava, Object, Result};

pub fn invoke_with_throwable<R, F: FnOnce() -> R>(ctx: Context, f: F) -> Result<R> {
    let suppressed_throwable = unsafe { call!(v1_1, ctx, ExceptionOccurred) };
    if suppressed_throwable != null_mut() {
        unsafe { call!(v1_1, ctx, ExceptionClear) };
    }

    let r = f();

    let throwable = unsafe { call!(v1_1, ctx, ExceptionOccurred) };
    if throwable != null_mut() {
        unsafe { call!(v1_1, ctx, ExceptionClear) };
    }

    if suppressed_throwable != null_mut() {
        unsafe { call!(v1_1, ctx, Throw, suppressed_throwable) };
    }

    if throwable != null_mut() {
        Err(throwable)
    } else {
        Ok(r)
    }
}

pub trait InvokeType {
    unsafe fn call_method(ctx: Context, this: Object, method: jmethodID, args: &[jvalue]) -> Self;
    unsafe fn call_static_method(ctx: Context, class: Class, method: jmethodID, args: &[jvalue]) -> Self;
    unsafe fn get_field(ctx: Context, this: Object, field: jfieldID) -> Self;
    unsafe fn get_static_field(ctx: Context, class: Class, field: jfieldID) -> Self;
    unsafe fn set_field(ctx: Context, this: Object, field: jfieldID, value: Self);
    unsafe fn set_static_field(ctx: Context, class: Class, field: jfieldID, value: Self);
}

macro_rules! impl_invoke_output {
    ($rs_type:ty, $java_type:ident) => {
        paste! {
           impl InvokeType for $rs_type {
               unsafe fn call_method(ctx: Context, this: Object, method: jmethodID, args: &[jvalue]) -> Self {
                   call!(
                       v1_1,
                       ctx,
                       [<Call $java_type MethodA>],
                       this,
                       method,
                       args.as_ptr()
                   )
               }

               unsafe fn call_static_method(ctx: Context, class: Class, method: jmethodID, args: &[jvalue]) -> Self {
                   call!(
                       v1_1,
                       ctx,
                       [<CallStatic $java_type MethodA>],
                       class,
                       method,
                       args.as_ptr()
                   )
               }

               unsafe fn get_field(ctx: Context, this: Object, field: jfieldID) -> Self {
                   call!(v1_1, ctx, [<Get $java_type Field>], this, field)
               }

               unsafe fn get_static_field(ctx: Context, class: Class, field: jfieldID) -> Self {
                   call!(v1_1, ctx, [<GetStatic $java_type Field>], class, field)
               }

               unsafe fn set_field(ctx: Context, this: Object, field: jfieldID, value: Self) {
                   call!(v1_1, ctx, [<Set $java_type Field>], this, field, value)
               }

               unsafe fn set_static_field(ctx: Context, class: Class, field: jfieldID, value: Self) {
                   call!(v1_1, ctx, [<SetStatic $java_type Field>], class, field, value)
               }
           }
        }
    };
}

impl_invoke_output!(jboolean, Boolean);
impl_invoke_output!(jbyte, Byte);
impl_invoke_output!(jchar, Char);
impl_invoke_output!(jshort, Short);
impl_invoke_output!(jint, Int);
impl_invoke_output!(jlong, Long);
impl_invoke_output!(jfloat, Float);
impl_invoke_output!(jdouble, Double);
impl_invoke_output!(jobject, Object);

impl InvokeType for () {
    unsafe fn call_method(ctx: Context, this: Object, method: jmethodID, args: &[jvalue]) -> Self {
        call!(v1_1, ctx, CallVoidMethodA, this, method, args.as_ptr())
    }

    unsafe fn call_static_method(ctx: Context, class: Class, method: jmethodID, args: &[jvalue]) -> Self {
        call!(v1_1, ctx, CallStaticVoidMethodA, class, method, args.as_ptr())
    }

    unsafe fn get_field(_: Context, _: Object, _: jfieldID) -> Self {
        panic!("unsupported")
    }

    unsafe fn get_static_field(_: Context, _: Class, _: jfieldID) -> Self {
        panic!("unsupported")
    }

    unsafe fn set_field(_: Context, _: Object, _: jfieldID, _: Self) {
        panic!("unsupported")
    }

    unsafe fn set_static_field(_: Context, _: Class, _: jfieldID, _: Self) {
        panic!("unsupported")
    }
}

pub unsafe fn call_method<T: InvokeType, R: FromJava<T>>(
    ctx: Context,
    this: Object,
    method: jmethodID,
    args: &[jvalue],
) -> Result<R> {
    invoke_with_throwable(ctx, || T::call_method(ctx, this, method, args)).and_then(|o| R::from_java(o, ctx))
}

pub unsafe fn call_static_method<T: InvokeType, R: FromJava<T>>(
    ctx: Context,
    this: Object,
    method: jmethodID,
    args: &[jvalue],
) -> Result<R> {
    invoke_with_throwable(ctx, || T::call_static_method(ctx, this, method, args)).and_then(|o| R::from_java(o, ctx))
}

pub unsafe fn get_field<T: InvokeType, R: FromJava<T>>(ctx: Context, this: Object, field: jfieldID) -> Result<R> {
    invoke_with_throwable(ctx, || T::get_field(ctx, this, field)).and_then(|o| R::from_java(o, ctx))
}

pub unsafe fn get_static_field<T: InvokeType, R: FromJava<T>>(ctx: Context, class: Class, field: jfieldID) -> Result<R> {
    invoke_with_throwable(ctx, || T::get_static_field(ctx, class, field)).and_then(|o| R::from_java(o, ctx))
}

pub unsafe fn set_field<V: InvokeType>(ctx: Context, this: Object, field: jfieldID, value: V) -> Result<()> {
    invoke_with_throwable(ctx, || V::set_field(ctx, this, field, value))
}

pub unsafe fn set_static_field<V: InvokeType>(ctx: Context, class: Class, field: jfieldID, value: V) -> Result<()> {
    invoke_with_throwable(ctx, || V::set_static_field(ctx, class, field, value))
}

pub unsafe fn new_object<R: FromJava<jobject>>(ctx: Context, class: Class, constructor: jmethodID, args: &[jvalue]) -> Result<R> {
    invoke_with_throwable(ctx, || call!(v1_1, ctx, NewObjectA, class, constructor, args.as_ptr()))
        .and_then(|o| R::from_java(o, ctx))
}
