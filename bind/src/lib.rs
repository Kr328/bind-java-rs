use jni_sys::{jclass, jobject, jthrowable, JNIEnv};

pub use bind::*;
pub use bind_java_marco::bind_java;
pub use binding::*;
pub use frame::*;
pub use from::*;
pub use func::*;
pub use into::*;
pub use invoke::*;
pub use loader::*;

mod bind;
mod binding;
mod call;
mod frame;
mod from;
mod func;
mod into;
mod invoke;
mod loader;

pub type Context = *mut JNIEnv;
pub type Class = jclass;
pub type Object = jobject;
pub type Throwable = jthrowable;
pub type Result<T> = std::result::Result<T, Throwable>;
