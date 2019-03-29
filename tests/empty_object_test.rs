#[macro_use]
extern crate nom;
use nom::Err;
use nom::verbose_errors::Context;
use nom::ErrorKind;

extern crate json_parser_lib;

#[test]
fn brackets_only() {
    let brackets = "{}";
    let raw_content = brackets.as_bytes();
    let result = ::json_parser_lib::parsers::empty_object(raw_content);
    let input = &b""[..];
    let output = (&b"{"[..], &b"}"[..]);
    assert_eq!(result, Ok((input, output)));
}

#[test]
fn whitespace_between_brackets() {
    let brackets = "{        \t\n}";
    let raw_content = brackets.as_bytes();
    let result = ::json_parser_lib::parsers::empty_object(raw_content);
    let input = &b""[..];
    let output = (&b"{"[..], &b"}"[..]);
    assert_eq!(result, Ok((input, output)));
}

#[test]
fn whitespace_around_brackets() {
    let brackets = "        \t\n{}        \t\n";
    let raw_content = brackets.as_bytes();
    let result = ::json_parser_lib::parsers::empty_object(raw_content);
    let input = &b"        \t\n"[..];
    let output = (&b"{"[..], &b"}"[..]);
    assert_eq!(result, Ok((input, output)));
}

#[test]
fn whitespace_around_and_between_brackets() {
    let brackets = "        \t\n{        \t\n}        \t\n";
    let raw_content = brackets.as_bytes();
    let result = ::json_parser_lib::parsers::empty_object(raw_content);
    let input = &b"        \t\n"[..];
    let output = (&b"{"[..], &b"}"[..]);
    assert_eq!(result, Ok((input, output)));
}
