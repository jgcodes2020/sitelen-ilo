use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    character::char,
};

use crate::{
    ast::object::Literal,
    parse::{
        Span,
        error::{NomErrorKind, ParseError, ParseResult},
    },
};

const ERR_OPEN_QUOTE_UNESCAPED: &str = "unescaped opening quote ['「'] in literal";
const HELP_QUOTE_ESCAPING: &str =
    "quotes should be escaped by doubling them up, i.e. like this: `「「` or `」」`";

/// Parses the quoted portion of a *toki* literal.
pub(super) fn toki_quoted(input: Span) -> ParseResult<Literal> {
    let mut result = String::new();

    // open quote
    let (mut remain, _) = char('「').parse_complete(input)?;
    loop {
        // read until open or close quote
        let (remain1, chunk) = take_till(|c| ['「', '」'].contains(&c))(remain)?;
        result.push_str(&chunk);

        // fail if we hit EOF
        if remain1.len() == 0 {
            return Err(ParseError::new(remain1, NomErrorKind::Eof.description())
                .with_help(HELP_QUOTE_ESCAPING)
                .into_failure());
        }

        // next escaped or closing quote
        let (remain2, quote) =
            alt((tag("「「"), tag("」」"), tag("「"), tag("」"))).parse(remain1)?;
        match &quote as &str {
            // escaped
            "「「" => result.push_str("「"),
            "」」" => result.push_str("」"),
            // unescaped
            "「" => {
                return Err(ParseError::new(remain1, ERR_OPEN_QUOTE_UNESCAPED)
                    .with_help(HELP_QUOTE_ESCAPING)
                    .into_failure());
            }
            "」" => {
                // end of string, take the quote and break
                remain = remain2;
                break;
            }
            _ => unreachable!(), // if we haven't hit EOF, one of these tags will match.
        }
        remain = remain2;
    }

    Ok((remain, Literal::Toki(result)))
}

#[cfg(test)]
mod tests {
    use sitelen_ilo_macros::sp;

    use crate::{
        ast::object::Literal,
        parse::{Span, object::toki_quoted},
    };

    fn check_valid(test_val: &str, val: &str) {
        let mut span: Span = Span::new(test_val);

        let value: Literal;
        (span, value) = toki_quoted(span).expect("parser should not error");

        assert_eq!(value, Literal::Toki(val.into()));
        assert!(span.is_empty());
    }
    fn check_invalid(test_val: &str) {
        let span: Span = Span::new(test_val);
        let err = toki_quoted(span).expect_err("parser should fail");
        assert!(!err.is_incomplete());
    }

    #[test]
    fn test_no_escapes() {
        check_valid(
            "「me when the toki isn't pona」",
            "me when the toki isn't pona",
        );
        check_valid(
            sp!("<sitelen li pona ala la mi:>"),
            sp!("sitelen li pona ala la mi:"),
        );
    }

    #[test]
    fn test_close_escape() {
        check_valid(
            sp!("<mi ken pini ala sama ni >> pona la ni li pakala ala>"),
            sp!("mi ken pini ala sama ni > pona la ni li pakala ala"),
        );
    }

    #[test]
    fn test_open_escape() {
        check_valid(
            sp!("<mi wile e ni <<ni li pakala ala>>>"),
            sp!("mi wile e ni <ni li pakala ala>"),
        );
        check_invalid(
            sp!("<mi wile e ni <ni li pakala wawa>>>"),
        );
    }
}
