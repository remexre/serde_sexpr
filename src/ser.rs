use crate::{Error, Result, Value};
use serde::ser::Serialize;
use std::io::Write;

/// Serialize the given data structure as an S-Expression into the writer.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// let mut c = Cursor::new(Vec::new());
/// let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
/// serde_sexpr::to_writer(&mut c, &value).unwrap();
/// let expected: &[u8] = b"(Hello! |Goodbye, world!| |)\\|(|)";
/// assert_eq!(c.into_inner(), expected);
/// ```
pub fn to_writer<T: Serialize + ?Sized, W: Write>(mut writer: W, value: &T) -> Result<()> {
    let bytes = to_vec(value)?;
    writer.write_all(&bytes)?;
    Ok(())
}

/// Serialize the given data structure as an S-Expression in a byte vector.
///
/// # Examples
///
/// ```
/// let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
/// assert_eq!(
///     serde_sexpr::to_vec(&value).unwrap(),
///     "(Hello! |Goodbye, world!| |)\\|(|)".as_bytes(),
/// );
/// ```
pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    to_string(value).map(String::into_bytes)
}

/// Serialize the given data structure as an S-Expression in a string.
///
/// # Examples
///
/// ```
/// let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
/// assert_eq!(
///     serde_sexpr::to_string(&value).unwrap(),
///     "(Hello! |Goodbye, world!| |)\\|(|)",
/// );
/// ```
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    to_value(value).map(|v| v.to_string())
}

/// Serialize the given data structure as an S-Expression in a `serde_sexpr::Value`.
///
/// # Examples
///
/// ```
/// let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
/// assert_eq!(
///     serde_sexpr::to_value(&value).unwrap(),
///     serde_sexpr::sexpr!( ("Hello!" "Goodbye, world!" ")|(") ),
/// );
/// ```
pub fn to_value<T: Serialize + ?Sized>(value: &T) -> Result<Value> {
    unimplemented!()
}

/// A serializer for S-Expressions.
#[derive(Debug)]
pub struct Serializer;

/*
impl serde::ser::Serializer for Serializer {
    type Ok = Value;

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Value> {
        if v {
            Ok(sexpr!(true))
        } else {
            Ok(sexpr!(false))
        }
    }
}
*/
