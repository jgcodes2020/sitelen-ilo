//! Parser for [*nasin nanpa pona*](https://sona.pona.la/wiki/nasin_nanpa_pona)

use nom::{Input, Parser, character::char};
use sitelen_ilo_macros::sp_c;

use crate::{ast::object::Literal, parse::{
    error::{nom_force_fatal, ParseError, ParseResult}, Span
}};

const ERR_NNP_TOO_LARGE: &str = "value out of range for nanpa";
const ERR_NNP_FAILED_MATCH: &str = "failed to match nasin nanpa pona number";

/// Parses [*nasin nanpa pona*](https://sona.pona.la/wiki/nasin_nanpa_pona), the de-facto standard
/// for writing large numbers quickly in *toki pona*.
fn nasin_nanpa_pona(input: Span) -> ParseResult<i64> {
    let mut result = 0i64;
    let mut expect_end = false;
    let mut add_ale = false;
    let mut end_idx: Option<usize> = None;

    for (i, c) in input.char_indices() {
        // To remain backwards-compatible with nasin pu, nasin nanpa pona
        // makes ale additive if used first.
        if i == 0 {
            add_ale = c == sp_c!("ale");
        }

        if expect_end {
            end_idx = Some(i);
            break;
        }
        match c {
            sp_c!("ale") => {
                // check if ale was the first character
                if add_ale {
                    result = result
                        .checked_add(100)
                        .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?
                } else {
                    result = result
                        .checked_mul(100)
                        .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?
                }
            }
            sp_c!("mute") => {
                result = result
                    .checked_add(20)
                    .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?;
            }
            sp_c!("luka") => {
                result = result
                    .checked_add(5)
                    .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?;
            }
            sp_c!("tu") => {
                result = result
                    .checked_add(2)
                    .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?;
            }
            sp_c!("wan") => {
                result = result
                    .checked_add(1)
                    .ok_or(ParseError::new(input, ERR_NNP_TOO_LARGE).into_failure())?;
            }
            sp_c!("weka") => {
                // number will end immediately after weka
                result = -result;
                expect_end = true;
            }
            _ => {
                end_idx = Some(i);
                break;
            }
        }
    }

    let remain = match end_idx {
        Some(0) => {
            return Err(ParseError::new(input, ERR_NNP_FAILED_MATCH).into_failure());
        }
        Some(i) => input.take_from(i),
        None => input.take_from(input.len()),
    };

    Ok((remain, result))
}

/// Parses the quoted portion of a *nanpa* literal.
pub(super) fn nanpa_quoted(input: Span) -> ParseResult<Literal> {
    let (input1, _) = char('「').parse(input)?;
    let (input2, value) = nasin_nanpa_pona(input1)?;
    let (input3, _) = char('」').parse(input2).map_err(nom_force_fatal)?;
    Ok((input3, Literal::Nanpa(value)))
}
