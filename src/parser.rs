use crate::{needs_quoting, Error, Value};
use nom::{
    alt_complete, anychar, char, delimited, do_parse, eof, many0, map, named, separated_list,
    take_while, take_while1, types::CompleteStr,
};
use std::{borrow::Cow, str::FromStr};

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Value, Error> {
        match parser(CompleteStr(s)) {
            Ok((rest, value)) => {
                if rest.is_empty() {
                    Ok(value)
                } else {
                    Err(Error::ParseTrailing)
                }
            }
            Err(_) => Err(Error::ParseFailed),
        }
    }
}

named!(parser<CompleteStr, Value>, do_parse!(ws >> v: value >> ws >> eof!() >> (v)));
named!(value<CompleteStr, Value>, alt_complete!( list | escaped_sym | unescaped_sym ));

named!(list<CompleteStr, Value>, map!(delimited!(char!('('), list_body, char!(')')), Value::List));
named!(list_body<CompleteStr, Vec<Value>>, separated_list!(ws, value));

named!(escaped_sym<CompleteStr, Value>,
    map!(delimited!(char!('|'), many0!(sym_chs), char!('|')),
         |s| Value::Sym(s.into_iter().collect())));
named!(unescaped_sym<CompleteStr, Value>,
    map!(take_while1!(doesnt_need_quoting),
         |s| Value::Sym(s.to_string())));

named!(sym_chs<CompleteStr, Cow<str>>, alt_complete!(escaped_ch | unescaped_chs));
named!(escaped_ch<CompleteStr, Cow<str>>, do_parse!(
    char!('\\') >>
    ch: anychar >>
    (Cow::Owned(Some(ch).into_iter().collect()))));
named!(unescaped_chs<CompleteStr, Cow<str>>,
    map!(take_while1!(doesnt_need_quoting), |CompleteStr(s)| Cow::Borrowed(s)));

named!(ws<CompleteStr, ()>, map!(take_while!(char::is_whitespace), |_| ()));

fn doesnt_need_quoting(ch: char) -> bool {
    !needs_quoting(ch)
}
