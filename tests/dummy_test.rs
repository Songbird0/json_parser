#![feature(trace_macros)]

#[macro_use]
extern crate nom;

trace_macros!(true);
named!( get_a<&[u8], Vec<&[u8]>>, dbg!(
  complete!(
    many0!(
      tag!("a")
    )
  )
));
trace_macros!(false);

#[test]
fn dummy_test() {
    let wrapped_raw_content : Result<(&[u8], Vec<&[u8]>), ()> = get_a( "aaa".as_bytes() )
        .map_err( |e| {
            match ::std::dbg!( e ) {
                nom::Err::Error(c) => {
                    match c {
                        nom::Context::Code(a_list, e) => {
                            ::std::dbg!( String::from_utf8_lossy( a_list ) );
                        },
                        nom::Context::List(vec) => {
                            ::std::dbg!( vec );
                        }
                    };
                },
                default @ _ => {
                    ::std::dbg!( default );
                },
            };
        } );
    assert_eq!(wrapped_raw_content, Err(()));
}