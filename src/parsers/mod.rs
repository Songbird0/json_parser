use nom::{is_alphanumeric, is_space};

/*named!( pub empty_object<&[u8], (&[u8], &[u8])>,
  dbg!( ws!( tuple!( tag!("{"), tag!("}") ) ) )
);*/

/*
        complete!(
          tuple!(
            ws!(
              tag!("{")
            ),
            ws!(
              tag!("}")
            )
          )
        )
*/

pub fn empty_object(input: &[u8]) -> nom::IResult<&[u8], (&[u8], &[u8]), u32> {
    let (i, o) = try_parse!(
      input,
      dbg!(
        do_parse!(
          opening: ws!(tag!("{")) >>
          ending: tag!("}") >>
          ((opening, ending))
        )
      )
    );
    Ok((i, o))
}

named!( pub string,
  dbg!( delimited!( char!('"'), ws!( take_while!( is_alphanumeric ) ), char!('"') ) )
);
