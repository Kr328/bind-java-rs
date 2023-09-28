#[macro_export]
macro_rules! system_fn {
    (|| $body:block) => {
        {
            extern "system" fn __func() { $body }

            __func
        }
    };
    (|| -> $ret:ty $body:block) => {
        {
            extern "system" fn __func() -> $ret $body

            __func
        }
    };
    (|$($name:tt:$typ:ty),*| $body:block) => {
        {
            extern "system" fn __func($($name:$typ),*) $body

            __func
        }
    };
    (|$($name:tt:$typ:ty),*| -> $ret:ty $body:block) => {
        {
            extern "system" fn __func($($name:$typ),*) -> $ret $body

            __func
        }
    };
}
