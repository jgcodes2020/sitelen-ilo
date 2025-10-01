use nom::{
    Parser,
    branch::alt,
    bytes::complete::take_while,
    character::{anychar, complete::char},
    combinator::{map, value},
};
use sitelen_ilo_macros::sp_c;

use crate::{
    ast::object::{NiRef, Object, PrimitiveType, Variable},
    parse::{
        Span,
        error::{ParseError, ParseResult, nom_force_failure},
        object::{lon::lon_quoted, nanpa::nanpa_quoted, toki::toki_quoted},
        util::CharSitelenPonaExt,
    },
};

mod lon;
mod nanpa;
mod toki;

const ERR_MISS_OPEN_QUOTE: &str = "missing opening quote ['「'] for literal";
const ERR_MISS_CLOSE_QUOTE: &str = "missing closing quote ['」'] for literal";

const ERR_MISS_OPEN_CART: &str = "missing cartouche open ['󱦐'] for variable";
const ERR_MISS_CLOSE_CART: &str = "missing cartouche close ['󱦑'] for variable";

const ERR_VAR_FORMAT: &str =
    "identifiers must start with a word and contain only sitelen pona words, dots, and colons";

/// Parses a cartouche (i.e. identifier).
pub(super) fn cartouche(input: Span) -> ParseResult<String> {
    // cartouche open
    let (input1, _) =
        char(sp_c!("["))(input).map_err(ParseError::override_reason(ERR_MISS_OPEN_CART))?;
    // first character
    let (input2, c0) = anychar(input1).and_then(|(input2, c)| {
        if c.is_sp_word() {
            Ok((input2, c))
        } else {
            Err(ParseError::new(input2, ERR_VAR_FORMAT).into_failure())
        }
    })?;
    // remaining characters
    let (input3, rest) =
        take_while(|c: char| c.is_sp_word() || matches!(c, sp_c!(".") | sp_c!(":")))(input2)
            .map_err(ParseError::override_reason(ERR_VAR_FORMAT))
            .map_err(nom_force_failure)?;
    // cartouche close
    let (input4, _) = char(sp_c!("]"))(input3).map_err(nom_force_failure)?;

    let value = format!("{}{}", c0, rest);
    Ok((input4, value))
}

/// Parses an object of some type.
pub(super) fn object(input: Span) -> ParseResult<Object> {
    // get type
    let (input1, ptype) = alt((
        value(PrimitiveType::Toki, char(sp_c!("toki"))),
        value(PrimitiveType::Nanpa, char(sp_c!("nanpa"))),
        value(PrimitiveType::Lon, char(sp_c!("lon"))),
    ))
    .parse_complete(input)?;

    // derive parsers for ni, cartouche, and literal
    let parse_ni = value(Object::Ni(NiRef { ptype }), char(sp_c!("ni")));
    let parse_cartouche = map(cartouche, |name| Object::Variable(Variable { ptype, name }));
    let parse_quoted = map(
        match ptype {
            PrimitiveType::Toki => toki_quoted,
            PrimitiveType::Nanpa => nanpa_quoted,
            PrimitiveType::Lon => lon_quoted,
        },
        |lit| Object::Literal(lit),
    );

    // parse for the corresponding type
    let (input2, out) = alt((parse_ni, parse_cartouche, parse_quoted)).parse(input1)?;

    Ok((input2, out))
}

#[cfg(test)]
mod tests {
    use sitelen_ilo_macros::sp;

    use crate::{
        ast::object::{Literal, NiRef, Object, PrimitiveType, Variable},
        parse::{Span, object::object},
    };

    fn check_valid(test_val: &str, val: Object) {
        let mut span: Span = Span::new(test_val);

        let value: Object;
        (span, value) = object(span).expect("parser should not error");

        assert_eq!(value, val);
        assert!(span.is_empty());
    }

    #[test]
    fn test_valid() {
        check_valid(
            sp!("lon ni"),
            Object::Ni(NiRef {
                ptype: PrimitiveType::Lon,
            }),
        );
        check_valid(
            sp!("nanpa [suli:]"),
            Object::Variable(Variable {
                ptype: PrimitiveType::Nanpa,
                name: sp!("suli:").into(),
            }),
        );
        check_valid(
            sp!("toki <jan pi pakala suli>"),
            Object::Literal(Literal::Toki(sp!("jan pi pakala suli").into())),
        );
    }
}
