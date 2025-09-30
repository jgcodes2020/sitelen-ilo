use nom::{
    branch::alt, bytes::complete::{tag, take_till}, character::{anychar, char}, Parser
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
pub fn toki_quoted(input: Span) -> ParseResult<Literal> {
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
                (remain, _) = anychar(remain2)?;
                break;
            },
            _ => unreachable!(), // if we haven't hit EOF, one of these tags will match.
        }
        remain = remain2;
    }

    Ok((remain, Literal::Toki(result)))
}
