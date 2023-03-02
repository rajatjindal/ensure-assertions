mod error;
pub use error::*;

#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {{
        use anyhow::ensure;
        use $crate::error_msg;
        ensure!(*left_val == *right_val, error_msg("!=", &*left_val, &*right_val, None));
    }};
    ($cond:expr, $($arg:tt)+) => {{
        use anyhow::ensure;
        use $crate::error_msg;
        ensure!(*left_val == *right_val, error_msg("==", &*left_val, &*right_val, Some(format_args!($($arg)+))));
    }};
}

#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                use anyhow::ensure;
                use $crate::error_msg;
                ensure!(*left_val == *right_val, error_msg("==", &*left_val, &*right_val, None));
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                use anyhow::ensure;
                use $crate::error_msg;
                ensure!(*left_val == *right_val, error_msg("==", &*left_val, &*right_val, Some(format_args!($($arg)+))))
            }
        }
    }};
}

#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                use anyhow::ensure;
                use $crate::error_msg;
                ensure!(*left_val != *right_val, error_msg("!=", &*left_val, &*right_val, None));
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                use anyhow::ensure;
                use $crate::error_msg;
                ensure!(*left_val != *right_val, error_msg("!=", &*left_val, &*right_val, Some(format_args!($($arg)+))))
            }
        }
    });
}