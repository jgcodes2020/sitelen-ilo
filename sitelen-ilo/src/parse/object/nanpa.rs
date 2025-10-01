//! Parser for [*nasin nanpa pona*](https://sona.pona.la/wiki/nasin_nanpa_pona)

use nom::{Input, Parser, character::char};
use sitelen_ilo_macros::sp_c;

use crate::{ast::object::Literal, parse::{
    error::{nom_force_failure, ParseError, ParseResult}, Span
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
        if i == 0 {
            match c {
                sp_c!("ala") => {
                    // special case: ala
                    expect_end = true;
                    continue;
                }
                sp_c!("weka") => {
                    // special case: weka can't be first
                    return Err(ParseError::new(input, ERR_NNP_FAILED_MATCH).into_failure());
                }
                _ => ()
            }
            // To remain backwards-compatible with nasin pu, nasin nanpa pona
            // makes ale additive if used first.
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
    let (input1, _) = char('「').parse_complete(input)?;
    let (input2, value) = nasin_nanpa_pona(input1)?;
    let (input3, _) = char('」').parse_complete(input2).map_err(nom_force_failure)?;
    Ok((input3, Literal::Nanpa(value)))
}

#[cfg(test)]
mod tests {
    use sitelen_ilo_macros::sp;

    use crate::{ast::object::Literal, parse::{object::nanpa::nanpa_quoted, Span}};

    fn check_valid(test_val: &str, num: i64) {
        let mut span: Span = Span::new(test_val);
        
        let value: Literal;
        (span, value) = nanpa_quoted(span).expect("parser should not error");

        assert_eq!(value, Literal::Nanpa(num));
        assert!(span.is_empty());
    }
    fn check_invalid(test_val: &str) {
        let span: Span = Span::new(test_val);
        let err = nanpa_quoted(span).expect_err("parser should fail");
        assert!(!err.is_incomplete());
    }

    #[test]
    fn test_zero() {
        check_valid(sp!("<ala>"), 0);
    }

    #[test]
    fn test_pos_units() {
        check_valid(sp!("<wan>"), 1);
        check_valid(sp!("<tu>"), 2);
        check_valid(sp!("<luka>"), 5);
        check_valid(sp!("<mute>"), 20);
    }

    #[test]
    fn test_pos_small() {
        check_valid(sp!("<luka luka>"), 10);
        check_valid(sp!("<luka luka luka>"), 15);
        check_valid(sp!("<mute luka tu wan>"), 28);
        check_valid(sp!("<mute mute tu>"), 42);
        check_valid(sp!("<mute mute mute luka tu tu>"), 69);
    }

    #[test]
    fn test_pos_fallback() {
        check_valid(sp!("<ale wan>"), 101);
        check_valid(sp!("<ale mute luka tu wan>"), 128);
        check_valid(sp!("<ale ale ale ale mute>"), 420);
    }

    #[test]
    fn test_pos_nnp() {
        check_valid(sp!("<luka luka luka tu tu ale mute mute mute mute tu tu>"), 1984);
        check_valid(sp!("<wan ale ale ale>"), 1_000_000);
        check_valid(sp!("<luka wan ale mute mute mute mute luka luka tu tu ale mute>"), 69420);
    }

    #[test]
    fn test_neg_units() {
        check_valid(sp!("<wan weka>"), -1);
        check_valid(sp!("<tu weka>"), -2);
        check_valid(sp!("<luka weka>"), -5);
        check_valid(sp!("<mute weka>"), -20);
    }

    #[test]
    fn test_neg_small() {
        check_valid(sp!("<luka luka weka>"), -10);
        check_valid(sp!("<luka luka luka weka>"), -15);
        check_valid(sp!("<mute luka tu wan weka>"), -28);
        check_valid(sp!("<mute mute tu weka>"), -42);
        check_valid(sp!("<mute mute mute luka tu tu weka>"), -69);
    }

    #[test]
    fn test_neg_fallback() {
        check_valid(sp!("<ale wan weka>"), -101);
        check_valid(sp!("<ale mute luka tu wan weka>"), -128);
        check_valid(sp!("<ale ale ale ale mute weka>"), -420);
    }

    #[test]
    fn test_neg_nnp() {
        check_valid(sp!("<luka luka luka tu tu ale mute mute mute mute tu tu weka>"), -1984);
        check_valid(sp!("<luka wan ale mute mute mute mute luka luka tu tu ale mute weka>"), -69420);
        check_valid(sp!("<wan ale ale ale weka>"), -1_000_000);
    }

    #[test]
    fn test_failures() {
        check_invalid(sp!("<wan"));
        check_invalid(sp!("<wan suli>"));
        check_invalid(sp!("<o moli e mi>"));
        check_invalid(sp!("<ala weka>"));
        check_invalid(sp!("<wan weka suli a>"));
        check_invalid(sp!("<weka>"));
    }
}