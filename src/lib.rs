mod error;

pub use error::*;

#[macro_export]
macro_rules! panic {
    () => {{
        Err::<(), _>($crate::ErrorBuilder::new()
            .file(file!())
            .line(line!())
            .column(column!())
            .build())
    }};
    ($($arg:tt)+) => {{
        Err::<(), _>($crate::ErrorBuilder::new()
            .file(file!())
            .line(line!())
            .column(column!())
            .message(format!($($arg)+)))
    }};
}

#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {{
        if $cond {
            Ok(())
        } else {
            Err($crate::ErrorBuilder::new()
                .file(file!())
                .line(line!())
                .column(column!())
                .build())
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if $cond {
            Ok(())
        } else {
            Err($crate::ErrorBuilder::new()
                .file(file!())
                .line(line!())
                .column(column!())
                .message(format!($($arg)+))
                .build())
        }
    }};
}

#[macro_export]
macro_rules! assert_eq2 {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                use anyhow::ensure;
                use $crate::error_msg;
                ensure!(*left_val == *right_val, None);
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
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val  {
                    Err($crate::ErrorBuilder::new()
                        .file(file!())
                        .line(line!())
                        .column(column!())
                        .build())
                } else {
                    Ok(())
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    Err($crate::ErrorBuilder::new()
                        .file(file!())
                        .line(line!())
                        .column(column!())
                        .message(format!($($arg)+))
                        .build())
                } else {
                    Ok(())
                }
            }
        }
    });
}