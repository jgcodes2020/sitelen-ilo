use nom::{branch::alt, character::char, combinator::value, Parser};
use sitelen_ilo_macros::sp_c;

use crate::{ast::object::Literal, parse::{error::{nom_force_fatal, ParseResult}, Span}};

/// Parses the quoted portion of a *lon* literal.
fn lon_quoted(input: Span) -> ParseResult<Literal> {
    let (input1, _) = char('「').parse(input)?;
    let (input2, value) = alt([
        value(true, char(sp_c!("lon"))),
        value(false, char(sp_c!("ala"))),
    ]).parse(input1)?;
    let (input3, _) = char('」').parse(input2).map_err(nom_force_fatal)?;
    Ok((input3, Literal::Lon(value)))
}