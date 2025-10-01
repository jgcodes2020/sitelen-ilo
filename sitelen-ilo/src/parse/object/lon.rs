use nom::{branch::alt, character::char, combinator::value, Parser};
use sitelen_ilo_macros::sp_c;

use crate::{ast::object::Literal, parse::{error::{nom_force_failure, ParseError, ParseResult}, Span}};

const ERR_FAILED_LON_MATCH: &str = "`lon`-typed value may only take `lon` or `ala`";

/// Parses the quoted portion of a *lon* literal.
pub(super) fn lon_quoted(input: Span) -> ParseResult<Literal> {
    let (input1, _) = char('「').parse_complete(input)?;
    let (input2, value) = alt([
        value(true, char(sp_c!("lon"))),
        value(false, char(sp_c!("ala"))),
    ]).parse_complete(input1).map_err(ParseError::override_reason(ERR_FAILED_LON_MATCH)).map_err(nom_force_failure)?;
    let (input3, _) = char('」').parse_complete(input2).map_err(nom_force_failure)?;
    Ok((input3, Literal::Lon(value)))
}

#[cfg(test)]
mod tests {
    use sitelen_ilo_macros::sp;

    use crate::{ast::object::Literal, parse::{object::lon::lon_quoted, Span}};

    fn check_valid(test_val: &str, val: bool) {
        let mut span: Span = Span::new(test_val);
        
        let value: Literal;
        (span, value) = lon_quoted(span).expect("parser should not error");

        assert_eq!(value, Literal::Lon(val));
        assert!(span.is_empty());
    }
    fn check_invalid(test_val: &str) {
        let span: Span = Span::new(test_val);
        let err = lon_quoted(span).expect_err("parser should fail");
        assert!(!err.is_incomplete());
    }

    #[test]
    fn test_true() {
        check_valid(sp!("<lon>"), true);
    }

    #[test]
    fn test_false() {
        check_valid(sp!("<ala>"), false);
    }

    #[test]
    fn test_failure() {
        check_invalid(sp!("<lon"));
        check_invalid(sp!("<lon suli>"));
        check_invalid(sp!("<o moli e mi>"));
    }
}