use nom::{
    branch::alt, bytes::complete::{take, take_till}, error::Error, multi::{fold, separated_list0}, Parser
};

use crate::tables::{PUNCT_TABLE, SP_TABLE};


pub(crate) fn ws(input: &str) -> nom::IResult<&str, &str> {
    take_till(|c: char| !c.is_whitespace())(input)
}

pub(crate) fn word(input: &str) -> nom::IResult<&str, char> {
    let (input1, word) = take_till(|c: char| !c.is_ascii_lowercase())(input)?;

    let c = *SP_TABLE
        .get(word)
        .ok_or_else(|| nom::Err::Error(Error::new(input, nom::error::ErrorKind::OneOf)))?;
    return Ok((input1, c));
}

pub(crate) fn punct(input: &str) -> nom::IResult<&str, char> {
    let (input1, taken) = take(1usize)(input)?;
    let output = PUNCT_TABLE.get(taken).cloned();
    output
        .ok_or_else(|| nom::Err::Error(Error::new(input, nom::error::ErrorKind::OneOf)))
        .map(|c| (input1, c))
}

pub(crate) fn translatable(input: &str) -> nom::IResult<&str, char> {
    alt([
        word,
        punct
    ]).parse(input)
}

pub(crate) fn sitelen_lasina(input: &str) -> nom::IResult<&str, String> {
    let (input, _) = ws(input)?;
    separated_list0(ws, translatable).parse(input).map(|(input, chars)| (input, chars.into_iter().collect::<String>()))
}