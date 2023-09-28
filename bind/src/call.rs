#[macro_export]
macro_rules! call {
    ($target_api:ident, $ctx:expr, $func_name:ident) => {
        ((**($ctx)).$target_api.$func_name)($ctx)
    };
    ($target_api:ident, $ctx:expr, $func_name:ident,) => {
        ((**($ctx)).$target_api.$func_name)($ctx)
    };
    ($target_api:ident, $ctx:expr, $func_name:ident, $($args:expr),*) => {
        ((**($ctx)).$target_api.$func_name)($ctx, $($args),*)
    };
}
