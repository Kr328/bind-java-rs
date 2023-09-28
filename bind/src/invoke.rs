use std::ptr::null_mut;

use crate::{call, Context, Result};

#[macro_export]
#[doc(hidden)]
macro_rules! _invoke {
    ($ret:ty, $ctx:expr, $func_name:ident, $target:expr, $member_id:expr) => {{
        let _r = $crate::with_pushed_frame($ctx, 16, move || {
            $crate::invoke_with_throwable(
                $ctx,
                move || $crate::call!(v1_1, $ctx, $func_name, $target, $member_id)
            )
        })?;

        <$ret as $crate::FromJava<_>>::from_java(_r, $ctx)
    }};
    ($ret:ty, $ctx:expr, $func_name:ident, $this:expr, $method:expr,) => {
        $crate::_invoke!($ret, $ctx, $func_name, $this, $method)
    };
    ($ret:ty, $ctx:expr, $func_name:ident, $target:expr, $method_id:expr, $($args:ident),*) => {{
        let _r = $crate::with_pushed_frame($ctx, 16, move || {
            $(let $args = ($args).into_java($ctx)?;)*;

            $crate::invoke_with_throwable(
                $ctx,
                move || $crate::call!(v1_1, $ctx, $func_name, $target, $method_id, $($args),*)
            )
        })?;

        <$ret as $crate::FromJava<_>>::from_java(_r, $ctx)
    }};
}

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
