use crate::{Error, Result, Value};
use serde::de::{DeserializeOwned, DeserializeSeed, Deserializer, Visitor};
use std::{
    io::Read,
    str::{from_utf8, FromStr},
};

/// Deserialize an instance of `T` from an S-Expression in a reader.
///
/// # Examples
///
/// ```
/// # use std::io::Cursor;
/// let c = Cursor::new("(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)".as_bytes());
/// let value: Vec<String> = serde_sexpr::from_reader(c).unwrap();
/// assert_eq!(value, vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()]);
/// ```
pub fn from_reader<R: Read, T: DeserializeOwned>(mut reader: R) -> Result<T> {
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf)?;
    from_slice(&buf)
}

/// Deserialize an instance of `T` from an S-Expression in a slice.
///
/// # Examples
///
/// ```
/// let s = b"(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)";
/// let value: Vec<String> = serde_sexpr::from_slice(s).unwrap();
/// assert_eq!(value, vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()]);
/// ```
pub fn from_slice<T: DeserializeOwned>(slice: &[u8]) -> Result<T> {
    from_str(from_utf8(slice)?)
}

/// Deserialize an instance of `T` from an S-Expression in a string.
///
/// # Examples
///
/// ```
/// let s = "(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)";
/// let value: Vec<String> = serde_sexpr::from_str(s).unwrap();
/// assert_eq!(value, vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()]);
/// ```
pub fn from_str<T: DeserializeOwned>(s: &str) -> Result<T> {
    let value = s.parse::<Value>()?;
    from_value(value)
}

/// Interpret a `serde_sexpr::Value` as an instance of type `T`.
pub fn from_value<T: DeserializeOwned>(value: Value) -> Result<T> {
    T::deserialize(value)
}

impl Value {
    /// Deserializes a `FromStr` from a `Sym`.
    fn deserialize_generic<T: FromStr>(self, name: &'static str) -> Result<T> {
        match &self {
            Value::List(_) => Err(Error::Invalid(name, self)),
            Value::Sym(s) => s.parse().map_err(|_| Error::Invalid(name, self)),
        }
    }
}

impl<'de> Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match &self {
            Value::List(_) => self.deserialize_seq(visitor),
            Value::Sym(_) => self.deserialize_str(visitor),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.deserialize_generic("bool")?)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.deserialize_generic("i8")?)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.deserialize_generic("i16")?)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.deserialize_generic("i32")?)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.deserialize_generic("i64")?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.deserialize_generic("u8")?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.deserialize_generic("u16")?)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.deserialize_generic("u32")?)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.deserialize_generic("u64")?)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.deserialize_generic("f32")?)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.deserialize_generic("f64")?)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_char(self.deserialize_generic("char")?)
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.deserialize_generic("string")?)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.deserialize_generic("string")?)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_generic("string")
            .map(String::into_bytes)
            .and_then(|bs| visitor.visit_byte_buf(bs))
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_generic("string")
            .map(String::into_bytes)
            .and_then(|bs| visitor.visit_byte_buf(bs))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match &self {
            Value::List(l) => {
                if l.is_empty() {
                    visitor.visit_none()
                } else {
                    visitor.visit_some(self)
                }
            }
            Value::Sym(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match &self {
            Value::List(l) => {
                if l.is_empty() {
                    visitor.visit_unit()
                } else {
                    Err(Error::Invalid("unit", self))
                }
            }
            Value::Sym(_) => Err(Error::Invalid("unit", self)),
        }
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self {
            Value::List(mut vs) => {
                vs.reverse();
                visitor.visit_seq(SeqAccess(vs))
            }
            Value::Sym(_) => Err(Error::Invalid("sequence", self)),
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self {
            Value::List(mut vs) => {
                vs.reverse();
                visitor.visit_map(MapAccess(vs, None))
            }
            Value::Sym(_) => Err(Error::Invalid("map", self)),
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_enum(EnumAccess(self))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }
}

struct EnumAccess(Value);

impl<'de> serde::de::EnumAccess<'de> for EnumAccess {
    type Error = Error;
    type Variant = VariantAccess;

    fn variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<(T::Value, Self::Variant)> {
        match self.0 {
            Value::List(mut vs) => {
                if vs.is_empty() {
                    Err(Error::Invalid("enum", Value::List(Vec::new())))
                } else {
                    seed.deserialize(vs.remove(0))
                        .map(|v| (v, VariantAccess(Some(vs))))
                }
            }
            v => seed.deserialize(v).map(|v| (v, VariantAccess(None))),
        }
    }
}

struct MapAccess(Vec<Value>, Option<Value>);

impl<'de> serde::de::MapAccess<'de> for MapAccess {
    type Error = Error;

    fn next_key_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        debug_assert!(self.1.is_none());
        match self.0.pop() {
            Some(Value::List(mut vs)) => {
                if vs.len() == 2 {
                    self.1 = vs.pop();
                    seed.deserialize(vs.pop().unwrap()).map(Some)
                } else {
                    Err(Error::Invalid("pair", Value::List(vs)))
                }
            }
            Some(v @ Value::Sym(_)) => Err(Error::Invalid("pair", v)),
            None => Ok(None),
        }
    }

    fn next_value_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<T::Value> {
        debug_assert!(self.1.is_some());
        seed.deserialize(self.1.take().unwrap())
    }
}

struct SeqAccess(Vec<Value>);

impl<'de> serde::de::SeqAccess<'de> for SeqAccess {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.0.is_empty() {
            Ok(None)
        } else {
            seed.deserialize(self.0.pop().unwrap()).map(Some)
        }
    }
}

struct VariantAccess(Option<Vec<Value>>);

impl<'de> serde::de::VariantAccess<'de> for VariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        debug_assert!(self.0.is_none());
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        debug_assert!(self.0.is_some());
        debug_assert_eq!(self.0.as_ref().unwrap().len(), 1);
        let val = self.0.unwrap().pop().unwrap();
        seed.deserialize(val)
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        debug_assert!(self.0.is_some());
        let val = Value::List(self.0.unwrap());
        val.deserialize_seq(visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        debug_assert!(self.0.is_some());
        let val = Value::List(self.0.unwrap());
        val.deserialize_map(visitor)
    }
}
