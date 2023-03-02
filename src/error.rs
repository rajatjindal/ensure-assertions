use std::fmt;

pub fn error_msg<T, U>(
    op: &str,
    left: &T,
    right: &U,
    args: Option<fmt::Arguments<'_>>,
) -> String
where
    T: fmt::Debug + ?Sized,
    U: fmt::Debug + ?Sized,
{
    match args {
        Some(args) => format!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`: {}"#,
            op, left, right, args
        ),
        None => format!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right,
        ),
    }
}