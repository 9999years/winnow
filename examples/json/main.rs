#![cfg(feature = "alloc")]

mod parser;

use winnow::error::convert_error;
use winnow::error::ErrorKind;
use winnow::error::VerboseError;
use winnow::prelude::*;

use parser::json;

fn main() {
  let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  } ";

  println!(
    "will try to parse valid JSON data:\n\n**********\n{}\n**********\n",
    data
  );

  // this will print:
  // Ok(
  //     Object(
  //         {
  //             "b": Array(
  //                 [
  //                     Str(
  //                         "x",
  //                     ),
  //                     Str(
  //                         "y",
  //                     ),
  //                     Num(
  //                         12.0,
  //                     ),
  //                 ],
  //             ),
  //             "c": Object(
  //                 {
  //                     "hello": Str(
  //                         "world",
  //                     ),
  //                 },
  //             ),
  //             "a": Num(
  //                 42.0,
  //             ),
  //         },
  //     ),
  // )
  println!(
    "parsing a valid file:\n{:#?}\n",
    // `finish` handles boilerplate from calling a parser
    json::<(&str, ErrorKind)>(data).finish()
  );

  let data = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { 1\"hello\" : \"world\"
  }
  } ";

  println!(
    "will try to parse invalid JSON data:\n\n**********\n{}\n**********\n",
    data
  );

  // here we use `(Input, ErrorKind)` as error type, which is used by default
  // if you don't specify it. It contains the position of the error and some
  // info on which parser encountered it.
  // It is fast and small, but does not provide much context.
  //
  // This will print:
  // basic errors - `json::<(&str, ErrorKind)>(data)`:
  // Err(
  //     (
  //       "1\"hello\" : \"world\"\n  }\n  } ",
  //       Char,
  //     ),
  // )
  println!(
    "basic errors - `json::<(&str, ErrorKind)>(data)`:\n{:#?}\n",
    json::<(&str, ErrorKind)>(data).finish()
  );

  // nom also provides `the `VerboseError<Input>` type, which will generate a sort
  // of backtrace of the path through the parser, accumulating info on input positions
  // and affected parsers.
  //
  // This will print:
  //
  // parsed verbose: Err(
  //     VerboseError {
  //         errors: [
  //             (
  //                 "1\"hello\" : \"world\"\n  }\n  } ",
  //                 Char(
  //                     '}',
  //                 ),
  //             ),
  //             (
  //                 "{ 1\"hello\" : \"world\"\n  }\n  } ",
  //                 Context(
  //                     "map",
  //                 ),
  //             ),
  //             (
  //                 "{ \"a\"\t: 42,\n  \"b\": [ \"x\", \"y\", 12 ] ,\n  \"c\": { 1\"hello\" : \"world\"\n  }\n  } ",
  //                 Context(
  //                     "map",
  //                 ),
  //             ),
  //         ],
  //     },
  // )
  println!(
    "parsed verbose: {:#?}",
    json::<VerboseError<&str>>(data).finish()
  );

  if let Err(e) = json::<VerboseError<&str>>(data).finish() {
    // here we use the `convert_error` function, to transform a `VerboseError<&str>`
    // into a printable trace.
    //
    // This will print:
    // verbose errors - `json::<VerboseError>(data)`:
    // 0: at line 2:
    //   "c": { 1"hello" : "world"
    //          ^
    // expected '}', found 1
    //
    // 1: at line 2, in map:
    //   "c": { 1"hello" : "world"
    //        ^
    //
    // 2: at line 0, in map:
    //   { "a" : 42,
    //   ^
    println!(
      "verbose errors - `json::<VerboseError>(data)`:\n{}",
      convert_error(data, e)
    );
  }

  assert!(json::<(&str, ErrorKind)>("null").finish().is_ok());
}
