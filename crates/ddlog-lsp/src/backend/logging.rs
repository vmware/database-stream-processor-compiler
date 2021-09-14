#[macro_export]
macro_rules! lsp_info {
    ($backend:expr, $message:literal) => {
        $crate::backend::Backend::info($backend, $message.to_owned())
    };

    ($backend:expr, $($arg:tt)+) => {
        $crate::backend::Backend::info($backend, ::std::format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_warn {
    ($backend:expr, $message:literal) => {
        $crate::backend::Backend::warn($backend, $message.to_owned())
    };

    ($backend:expr, $($arg:tt)+) => {
        $crate::backend::Backend::warn($backend, ::std::format!($($arg)*))
    };
}

#[macro_export]
macro_rules! lsp_error {
    ($backend:expr, $message:literal) => {
        $crate::backend::Backend::error($backend, $message.to_owned())
    };

    ($backend:expr, $($arg:tt)+) => {
        $crate::backend::Backend::error($backend, ::std::format!($($arg)*))
    };
}
