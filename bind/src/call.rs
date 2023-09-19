#[macro_export]
macro_rules! call {
    ($ctx:expr, $func_name:ident) => {
        (**($ctx)).$func_name.unwrap()($ctx)
    };
    ($ctx:expr, $func_name:ident,) => {
        (**($ctx)).$func_name.unwrap()($ctx)
    };
    ($ctx:expr, $func_name:ident, $($args:expr),*) => {
        (**($ctx)).$func_name.unwrap()($ctx, $($args),*)
    };
}
