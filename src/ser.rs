use crate::{Error, Result, Value};
use serde::ser::{
    Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use std::{fmt::Display, io::Write, str::from_utf8};

/// Serialize the given data structure as an S-Expression into the writer.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// let mut c = Cursor::new(Vec::new());
/// let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
/// serde_sexpr::to_writer(&mut c, &value).unwrap();
/// let expected: &[u8] = b"(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)";
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
///     "(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)".as_bytes(),
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
///     "(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)",
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
    value.serialize(Serializer)
}

/// A serializer for S-Expressions.
#[derive(Debug)]
pub struct Serializer;

/// Serializes a `Display`.
fn serialize_generic<T: Display>(value: T) -> Result<Value> {
    Ok(Value::Sym(value.to_string()))
}

impl serde::ser::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = ListSerializer;
    type SerializeTuple = ListSerializer;
    type SerializeTupleStruct = ListSerializer;
    type SerializeTupleVariant = ListSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = MapSerializer;

    fn serialize_bool(self, v: bool) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_char(self, v: char) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_str(self, v: &str) -> Result<Value> {
        serialize_generic(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value> {
        let v = from_utf8(v)?;
        serialize_generic(v)
    }

    fn serialize_none(self) -> Result<Value> {
        Ok(sexpr!(()))
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Value> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value> {
        Ok(sexpr!(()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        Ok(sexpr!(()))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        Ok(Value::Sym(variant.to_string()))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Value> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value> {
        value
            .serialize(self)
            .map(|value| Value::List(vec![Value::Sym(variant.to_string()), value]))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(ListSerializer(Vec::new()))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(ListSerializer(vec![Value::Sym(variant.to_string())]))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer(Vec::new(), None))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(MapSerializer(vec![Value::Sym(variant.to_string())], None))
    }
}

#[derive(Debug)]
pub struct ListSerializer(Vec<Value>);

impl SerializeSeq for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        to_value(value).map(|v| self.0.push(v))
    }

    fn end(self) -> Result<Value> {
        Ok(Value::List(self.0))
    }
}

impl SerializeTuple for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        to_value(value).map(|v| self.0.push(v))
    }

    fn end(self) -> Result<Value> {
        Ok(Value::List(self.0))
    }
}

impl SerializeTupleStruct for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        to_value(value).map(|v| self.0.push(v))
    }

    fn end(self) -> Result<Value> {
        Ok(Value::List(self.0))
    }
}

impl SerializeTupleVariant for ListSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        to_value(value).map(|v| self.0.push(v))
    }

    fn end(self) -> Result<Value> {
        Ok(Value::List(self.0))
    }
}

#[derive(Debug)]
pub struct MapSerializer(Vec<Value>, Option<Value>);

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        debug_assert!(self.1.is_none());
        self.1 = Some(to_value(value)?);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        debug_assert!(self.1.is_some());
        to_value(value).map(|v| {
            self.0.push(Value::List(vec![self.1.take().unwrap(), v]));
        })
    }

    fn end(self) -> Result<Value> {
        debug_assert!(self.1.is_none());
        Ok(Value::List(self.0))
    }
}

impl SerializeStruct for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.serialize_key(key)?;
        self.serialize_value(value)
    }

    fn end(self) -> Result<Value> {
        debug_assert!(self.1.is_none());
        Ok(Value::List(self.0))
    }
}

impl SerializeStructVariant for MapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.serialize_key(key)?;
        self.serialize_value(value)
    }

    fn end(self) -> Result<Value> {
        debug_assert!(self.1.is_none());
        Ok(Value::List(self.0))
    }
}
