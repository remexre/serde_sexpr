/// A macro for easily constructing `Value`s.
///
/// # Examples
///
/// ```
/// # use serde_sexpr::{sexpr, Value};
/// let a = sexpr!(foo);
/// let b = Value::Sym("foo".to_string());
/// assert_eq!(a, b);
///
/// let a = sexpr!("bar");
/// let b = Value::Sym("bar".to_string());
/// assert_eq!(a, b);
///
/// let a = sexpr!(12345);
/// let b = Value::Sym("12345".to_string());
/// assert_eq!(a, b);
///
/// let a = sexpr!(((a "bee") (c 0xd)));
/// let b = Value::List(vec![
///     Value::List(vec![
///         Value::Sym("a".to_string()),
///         Value::Sym("bee".to_string()),
///     ]),
///     Value::List(vec![
///         Value::Sym("c".to_string()),
///         Value::Sym("13".to_string()),
///     ]),
/// ]);
/// assert_eq!(a, b);
/// ```
#[macro_export]
macro_rules! sexpr {
    ( $s:ident ) => {
        $crate::Value::Sym(stringify!($s).to_string())
    };
    ( $s:literal ) => {
        $crate::Value::Sym($s.to_string())
    };
    ( ($($t:tt)*) ) => {
        $crate::Value::List(vec![
            $($crate::sexpr!($t)),*
        ])
    }
}
