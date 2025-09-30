use arrayvec::ArrayVec;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    error::{Error, ErrorKind},
};
use sitelen_ilo_macros::sp;

use crate::ast::object::Literal;

mod nanpa;
mod toki;
mod lon;

const ERR_MISS_OPEN_QUOTE: &str = "missing opening quote ['「'] for literal";
const ERR_MISS_CLOSE_QUOTE: &str = "missing closing quote ['」'] for literal";

const ERR_MISS_OPEN_CART: &str = "missing cartouche open ['󱦐'] for literal";
const ERR_MISS_CLOSE_CART: &str = "missing cartouche close ['󱦑'] for literal";

// pub fn quoted_string(mut input: &str) -> IResult<&str, Literal> {
//     let mut result = String::new();

//     // open quote
//     let (mut input1, _) = tag("「")(input)?;
//     loop {
//         let chunk: &str;
//         // read until open or close quote
//         (input1, chunk) = take_till(|c| c == '「' || c == '」')(input1)?;
//         result.push_str(chunk);

//         // next escaped or closing quote
//         let quote: &str;
//         (input1, quote) = alt((tag("「「"), tag("」」"), tag("「"), tag("」"))).parse(input1)?;
//         match quote {
//             // escaped
//             "「「" => result.push_str("「"),
//             "」」" => result.push_str("」"),
//             // unescaped
//             "「" => return Err(nom::Err::Failure(Error::new(input, ErrorKind::NoneOf))),
//             "」" => break,
//             _ => unreachable!()
//         }
//     }

//     Ok((input, Literal::Toki(result)))
// }