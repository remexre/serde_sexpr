use crate::needs_quoting;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An s-expression.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value {
    /// A list.
    List(Vec<Value>),

    /// A symbol.
    Sym(String),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            Value::List(l) => {
                write!(fmt, "(")?;
                let mut first = true;
                for x in l {
                    let sep = if first {
                        first = false;
                        ""
                    } else {
                        " "
                    };
                    write!(fmt, "{}{}", sep, x)?;
                }
                write!(fmt, ")")
            }
            Value::Sym(s) => {
                if s.chars().any(needs_quoting) || s.is_empty() {
                    write!(fmt, "|")?;
                    for ch in s.chars() {
                        let prefix = if needs_quoting(ch) { "\\" } else { "" };
                        write!(fmt, "{}{}", prefix, ch)?;
                    }
                    write!(fmt, "|")
                } else {
                    write!(fmt, "{}", s)
                }
            }
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Sym(s)
    }
}

impl From<Vec<Value>> for Value {
    fn from(l: Vec<Value>) -> Value {
        Value::List(l)
    }
}
