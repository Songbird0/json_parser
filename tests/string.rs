extern crate json_parser_lib;

fn generate_the_test(result: nom::IResult<&'static [u8], (char, &[u8], char), u32>,
                     input: &[u8],
                     output: (char, &[u8], char)) -> () {
    match result {
        Ok((i, o)) => {
            std::dbg!((String::from_utf8_lossy(i), String::from_utf8_lossy(o.1)));
            std::dbg!((String::from_utf8_lossy(input), String::from_utf8_lossy(output.1)));
            assert_eq!((i, o), (input, output));
        },
        Err(e) => {
            match e {
                nom::Err::Error(context) => {
                    match context {
                        nom::verbose_errors::Context::Code(input, error_kind) => {
                            std::dbg!(String::from_utf8_lossy(input));
                            panic!("{:?}", error_kind);
                        },
                        default @ _ => panic!(std::dbg!(default))
                    }
                },
                default @ _ => panic!(std::dbg!(default))
            };
        }
    };
}

#[test]
fn alphanum_characters_only() {
    let string = r#""alphaonly""#;
    let raw_content = string.as_bytes();

    /*match json_parser_lib::parsers::string(raw_content) {
        Ok((i, o)) => {
            let input = &b""[..];
            let output = ('"', &b"alphaonly"[..], '"');
            std::dbg!((String::from_utf8_lossy(i), String::from_utf8_lossy(o.1)));
            std::dbg!((String::from_utf8_lossy(input), String::from_utf8_lossy(output.1)));
            assert_eq!((i, o), (input, output));
        },
        Err(e) => {
            panic!(e);
        }
    };*/
    generate_the_test(
      json_parser_lib::parsers::string(raw_content),
      &b""[..], ('"', &b"alphaonly"[..], '"')
    );
}

#[test]
fn begin_with_whitespace() {
    let begin_with_whitespace = r#"" alphaonly""#;
    generate_the_test(
        json_parser_lib::parsers::string(begin_with_whitespace.as_bytes()),
        &b""[..],
        ('"', &b" alphaonly"[..], '"')
    );
}

#[test]
fn splitted() {
    let splitted = r#"" a l p h a o n l y ""#;
    generate_the_test(
        json_parser_lib::parsers::string(splitted.as_bytes()),
        &b""[..],
        ('"', &b" a l p h a o n l y "[..], '"')
    );
}

#[test]
fn end_with_white_space() {
    let end_with_white_space = r#""alphaonly ""#;
    generate_the_test(
        json_parser_lib::parsers::string(end_with_white_space.as_bytes()),
        &b""[..],
        ('"', &b"alphaonly "[..], '"')
    );
}

#[test]
fn trailing_whitespace() {
    let trailing_whitespace = "\" alphaonly \n\"";
    generate_the_test(
        json_parser_lib::parsers::string(trailing_whitespace.as_bytes()),
        &b""[..],
        ('"', &b" alphaonly \n"[..], '"')
    );
}

/*#[test]
fn separated_by_whitespace() {
    let begin_with_whitespace = r#"" alphaonly""#;
    let splitted = r#"" a l p h a o n l y ""#;
    let end_with_white_space = r#""alphaonly ""#;
    let trailing_whitespace = "\" alphaonly \n\"";

    generate_the_test(
        json_parser_lib::parsers::string(begin_with_whitespace.as_bytes()),
        &b""[..],
        ('"', &b" alphaonly"[..], '"')
    );
    generate_the_test(
        json_parser_lib::parsers::string(splitted.as_bytes()),
        &b""[..],
        ('"', &b" a l p h a o n l y "[..], '"')
    );
    generate_the_test(
        json_parser_lib::parsers::string(end_with_white_space.as_bytes()),
        &b""[..],
        ('"', &b"alphaonly "[..], '"')
    );
    generate_the_test(
        json_parser_lib::parsers::string(trailing_whitespace.as_bytes()),
        &b""[..],
        ('"', &b" alphaonly \n"[..], '"')
    );
}
*/