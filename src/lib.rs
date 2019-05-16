//! Serde support for S-Expressions.
//!
//! # Examples
//!
//! Any characters other than `(`, `)`, `|`, `\`, and whitespace can be used in a symbol. If you
//! want to use one of these characters, you can use an *escaped symbol,* which is surrounded by
//! `|` characters. Within an escaped symbol, these characters can be used, escaped by a backslash.
//!
//! ```
//! // Serialize!
//! let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
//! let sexpr = serde_sexpr::to_string(&value).unwrap();
//! assert_eq!(sexpr, "(Hello! |Goodbye,\\ world!| |\\)\\|\\(|)");
//!
//! // Deserialize!
//! let value2: Vec<String> = serde_sexpr::from_str(&sexpr).unwrap();
//! assert_eq!(value, value2);
//! ```
//!
//! Types are serialized as follows:
//!
//! ```
//! # use serde::Serialize;
//! # use std::collections::BTreeMap;
//! assert_eq!(serde_sexpr::to_string(&true).unwrap(), "true");
//! assert_eq!(serde_sexpr::to_string(&42i8).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42i16).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42i32).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42i64).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42u8).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42u16).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42u32).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&42u64).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&123.45f32).unwrap(), "123.45");
//! assert_eq!(serde_sexpr::to_string(&123.45f64).unwrap(), "123.45");
//! assert_eq!(serde_sexpr::to_string(&'%').unwrap(), "%");
//!
//! let s1: String = "s1".to_string();
//! let s2: &str = "s2";
//! assert_eq!(serde_sexpr::to_string(&s1).unwrap(), "s1");
//! assert_eq!(serde_sexpr::to_string(&s2).unwrap(), "s2");
//!
//! let b1: Vec<u8> = b"s1".to_vec();
//! let b2: &[u8] = b"s2";
//! assert_eq!(serde_sexpr::to_string(&b1).unwrap(), "(115 49)");
//! assert_eq!(serde_sexpr::to_string(&b2).unwrap(), "(115 50)");
//!
//! let o1: Option<&str> = Some("foo");
//! let o2: Option<&str> = None;
//! assert_eq!(serde_sexpr::to_string(&o1).unwrap(), "foo");
//! assert_eq!(serde_sexpr::to_string(&o2).unwrap(), "()");
//!
//! let mut map = BTreeMap::new();
//! map.insert(1, "yi");
//! map.insert(2, "er");
//! map.insert(3, "san");
//!
//! let t = (4, "five", b"SIX");
//!
//! let v = vec!["uno", "dos", "tres"];
//!
//! assert_eq!(serde_sexpr::to_string(&()).unwrap(), "()");
//! assert_eq!(serde_sexpr::to_string(&map).unwrap(), "((1 yi) (2 er) (3 san))");
//! assert_eq!(serde_sexpr::to_string(&t).unwrap(), "(4 five (83 73 88))");
//! assert_eq!(serde_sexpr::to_string(&v).unwrap(), "(uno dos tres)");
//!
//! #[derive(Serialize)]
//! struct UnitStruct;
//!
//! #[derive(Serialize)]
//! struct NewtypeStruct(i32);
//!
//! #[derive(Serialize)]
//! struct TupleStruct(i32, bool);
//!
//! #[derive(Serialize)]
//! struct Struct {
//!     foo: i32,
//!     bar: bool,
//! }
//!
//! #[derive(Serialize)]
//! enum Foo {
//!     UnitVariant,
//!     NewtypeVariant(i32),
//!     TupleVariant(i32, bool),
//!     StructVariant {
//!         foo: i32,
//!         bar: bool,
//!     }
//! }
//!
//! assert_eq!(serde_sexpr::to_string(&UnitStruct).unwrap(), "()");
//! assert_eq!(serde_sexpr::to_string(&NewtypeStruct(42)).unwrap(), "42");
//! assert_eq!(serde_sexpr::to_string(&TupleStruct(42, true)).unwrap(), "(42 true)");
//! assert_eq!(
//!     serde_sexpr::to_string(&Struct { foo: 42, bar: true }).unwrap(),
//!     "((foo 42) (bar true))"
//! );
//! assert_eq!(serde_sexpr::to_string(&Foo::UnitVariant).unwrap(), "UnitVariant");
//! assert_eq!(serde_sexpr::to_string(&Foo::NewtypeVariant(42)).unwrap(), "(NewtypeVariant 42)");
//! assert_eq!(
//!     serde_sexpr::to_string(&Foo::TupleVariant(42, true)).unwrap(),
//!     "(TupleVariant 42 true)"
//! );
//! assert_eq!(
//!     serde_sexpr::to_string(&Foo::StructVariant { foo: 42, bar: true }).unwrap(),
//!     "(StructVariant (foo 42) (bar true))"
//! );
//! ```
//!
//! And correspondingly deserialized:
//!
//! ```
//! # use serde::Deserialize;
//! # use std::collections::BTreeMap;
//! assert_eq!(serde_sexpr::from_str::<bool>("true").unwrap(), true);
//! assert_eq!(serde_sexpr::from_str::<i8>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<i16>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<i32>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<i64>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<u8>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<u16>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<u32>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<u64>("42").unwrap(), 42);
//! assert_eq!(serde_sexpr::from_str::<f32>("123.45").unwrap(), 123.45);
//! assert_eq!(serde_sexpr::from_str::<f64>("123.45").unwrap(), 123.45);
//! assert_eq!(serde_sexpr::from_str::<char>("%").unwrap(), '%');
//!
//! assert_eq!(serde_sexpr::from_str::<String>("s1").unwrap(), "s1");
//! assert_eq!(serde_sexpr::from_str::<Vec<u8>>("(115 49)").unwrap(), b"s1");
//!
//! assert_eq!(serde_sexpr::from_str::<Option<String>>("foo").unwrap(), Some("foo".to_string()));
//! assert_eq!(serde_sexpr::from_str::<Option<String>>("()").unwrap(), None);
//!
//! let mut map = BTreeMap::new();
//! map.insert(1, "yi".to_string());
//! map.insert(2, "er".to_string());
//! map.insert(3, "san".to_string());
//!
//! assert_eq!(serde_sexpr::from_str::<()>("()").unwrap(), ());
//! assert_eq!(
//!     serde_sexpr::from_str::<BTreeMap<i32, String>>("((1 yi) (2 er) (3 san))").unwrap(),
//!     map
//! );
//! assert_eq!(
//!     serde_sexpr::from_str::<(i32, String, Vec<u8>)>("(4 five (83 73 88))").unwrap(),
//!     (4, "five".to_string(), b"SIX".to_vec())
//! );
//! assert_eq!(
//!     serde_sexpr::from_str::<Vec<String>>("(uno dos tres)").unwrap(),
//!     vec!["uno", "dos", "tres"]
//! );
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct UnitStruct;
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct NewtypeStruct(i32);
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct TupleStruct(i32, bool);
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct Struct {
//!     foo: i32,
//!     bar: bool,
//! }
//!
//! #[derive(Debug, Deserialize, PartialEq)]
//! enum Foo {
//!     UnitVariant,
//!     NewtypeVariant(i32),
//!     TupleVariant(i32, bool),
//!     StructVariant {
//!         foo: i32,
//!         bar: bool,
//!     }
//! }
//!
//! assert_eq!(serde_sexpr::from_str::<UnitStruct>("()").unwrap(), UnitStruct);
//! assert_eq!(serde_sexpr::from_str::<NewtypeStruct>("42").unwrap(), NewtypeStruct(42));
//! assert_eq!(serde_sexpr::from_str::<TupleStruct>("(42 true)").unwrap(), TupleStruct(42, true));
//! assert_eq!(
//!     serde_sexpr::from_str::<Struct>("((foo 42) (bar true))").unwrap(),
//!     Struct { foo: 42, bar: true }
//! );
//! assert_eq!(serde_sexpr::from_str::<Foo>("UnitVariant").unwrap(), Foo::UnitVariant);
//! assert_eq!(serde_sexpr::from_str::<Foo>("(NewtypeVariant 42)").unwrap(), Foo::NewtypeVariant(42));
//! eprintln!("x");
//! assert_eq!(
//!     serde_sexpr::from_str::<Foo>("(TupleVariant 42 true)").unwrap(),
//!     Foo::TupleVariant(42, true)
//! );
//! eprintln!("x");
//! assert_eq!(
//!     serde_sexpr::from_str::<Foo>("(StructVariant (foo 42) (bar true))").unwrap(),
//!     Foo::StructVariant { foo: 42, bar: true }
//! );
//! ```
#![deny(
    bad_style,
    bare_trait_objects,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

mod de;
mod error;
#[macro_use]
mod macros;
mod parser;
mod ser;
#[cfg(test)]
mod tests;
mod value;

pub use crate::{
    de::{from_reader, from_slice, from_str, from_value},
    error::{Error, Result},
    ser::{to_string, to_value, to_vec, to_writer, Serializer},
    value::Value,
};

/// Returns whether the given character needs quoting.
fn needs_quoting(ch: char) -> bool {
    ch.is_whitespace() || "()|\\".contains(ch)
}
