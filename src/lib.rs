//! Serde support for S-Expressions.
//!
//! # Examples
//!
//! ```
//! // Serialize!
//! let value = vec!["Hello!".to_string(), "Goodbye, world!".to_string(), ")|(".to_string()];
//! let sexpr = serde_sexpr::to_string(&value).unwrap();
//! assert_eq!(sexpr, "(Hello! |Goodbye, world!| |)\\|(|)");
//!
//! // Deserialize!
//! let value2: Vec<String> = serde_sexpr::from_str(&sexpr).unwrap();
//! assert_eq!(value, value2);
//! ```
/*
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
*/

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
