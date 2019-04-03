use nom::is_alphanumeric;

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

// named!( pub string,
//  dbg!( delimited!( char!('"'), ws!( take_while!( is_alphanumeric ) ), char!('"') ) )
// );

named!(pub string<&[u8], (char, &[u8], char)>,
  dbg!(
    do_parse!(
      opening: char!('"') >>
      content: take_until!("\"") >>
      ending: char!('"') >>
      (opening, content, ending)
    )
  )
);
