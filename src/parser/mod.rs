use parcel::parsers::character::{any_character, eof, expect_character, expect_str, whitespace};
use parcel::prelude::v1::*;

pub mod ast;

#[derive(PartialEq)]
#[allow(unused)]
pub enum ParseErr {
    InvalidRule,
    Undefined(String),
}

impl std::fmt::Debug for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined(err) => write!(f, "undefined parse error: {}", err),
            Self::InvalidRule => write!(f, "provided rule is invalid",),
        }
    }
}

pub fn parse<S: AsRef<str>>(input: S) -> Result<ast::Definition, ParseErr> {
    let input: Vec<(usize, char)> = input.as_ref().chars().enumerate().collect();
    let result = match definition().parse(&input) {
        Ok(MatchStatus::Match { inner, .. }) => Ok(inner),
        Ok(MatchStatus::NoMatch { .. }) => Err(ParseErr::InvalidRule),
        Err(e) => Err(ParseErr::Undefined(format!("{:?}", e))),
    };

    result
}

fn definition<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::Definition> {
    parcel::one_or_more(directive()).map(ast::Definition)
}

fn directive<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::Directive> {
    parcel::left(parcel::join(
        directive_item().map(ast::Directive),
        parcel::join(
            expect_character(';'),
            parcel::or(parcel::one_or_more(whitespace()).map(|_| '\n'), eof),
        ),
    ))
}

fn directive_item<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::DirectiveItem> {
    parcel::or(gate_def().map(ast::DirectiveItem::GateDef), || {
        link_def().map(ast::DirectiveItem::LinkDef)
    })
}

fn gate_def<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::GateDef> {
    parcel::join(
        parcel::right(parcel::join(
            non_newline_whitespace_wrapped(expect_str("DEFINE")),
            non_newline_whitespace_wrapped(gate_identifier()),
        )),
        parcel::right(parcel::join(
            non_newline_whitespace_wrapped(expect_str("AS")),
            non_newline_whitespace_wrapped(gate_ty()),
        )),
    )
    .map(|(id, ty)| ast::GateDef::new(id, ty))
}

fn gate_ty<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::GateTy> {
    non_newline_whitespace_wrapped(parcel::one_of(vec![
        expect_str("not").map(|_| ast::GateTy::Not),
        expect_str("and").map(|_| ast::GateTy::And),
        expect_str("or").map(|_| ast::GateTy::Or),
        expect_str("xor").map(|_| ast::GateTy::Xor),
        expect_str("nand").map(|_| ast::GateTy::Nand),
        expect_str("nor").map(|_| ast::GateTy::Nor),
    ]))
}

fn link_def<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::LinkDef> {
    parcel::join(
        parcel::right(parcel::join(
            non_newline_whitespace_wrapped(expect_str("LINK")),
            non_newline_whitespace_wrapped(gate_identifier()),
        )),
        parcel::join(
            parcel::right(parcel::join(
                non_newline_whitespace_wrapped(expect_str("->")),
                non_newline_whitespace_wrapped(input_identifier()),
            )),
            parcel::right(parcel::join(
                non_newline_whitespace_wrapped(expect_str("OF")),
                non_newline_whitespace_wrapped(gate_identifier()),
            )),
        ),
    )
    .map(|(src, (input_id, dest))| ast::LinkDef::new(src, dest, input_id))
}

fn gate_identifier<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::GateIdentifier> {
    parcel::join(
        any_character().predicate(|c| c.is_ascii_lowercase()),
        parcel::one_or_more(parcel::or(
            any_character().predicate(|c| c.is_ascii_lowercase()),
            || expect_character('_'),
        )),
    )
    .map(|(head, tail)| [head].iter().chain(tail.iter()).copied().collect())
    // safe to call unchecked due to above guarantee
    .map(ast::GateIdentifier::try_new_unchecked)
}

fn input_identifier<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::InputIdentifier> {
    any_character()
        .predicate(|c| c.is_ascii_lowercase())
        // safe to call unchecked due to above guarantee
        .map(ast::InputIdentifier::try_new_unchecked)
}

fn non_newline_whitespace_wrapped<'a, P, B>(parser: P) -> impl Parser<'a, &'a [(usize, char)], B>
where
    B: 'a,
    P: Parser<'a, &'a [(usize, char)], B> + 'a,
{
    use parcel::parsers::character::non_newline_whitespace;

    parcel::right(parcel::join(
        parcel::zero_or_more(non_newline_whitespace()),
        parcel::left(parcel::join(
            parser,
            parcel::zero_or_more(non_newline_whitespace()),
        )),
    ))
}

#[cfg(test)]
mod tests {
    use super::ast::*;
    use super::*;

    #[test]
    fn should_parse_known_good_directive_varients() {
        let inputs = vec![
            // gate definition
            (
                "DEFINE gate AS and;",
                DirectiveItem::GateDef(GateDef::new(
                    GateIdentifier::try_new_unchecked("gate".to_string()),
                    GateTy::And,
                )),
            ),
            // link definition
            (
                "LINK src -> a OF dest;",
                DirectiveItem::LinkDef(LinkDef::new(
                    GateIdentifier::try_new_unchecked("src".to_string()),
                    GateIdentifier::try_new_unchecked("dest".to_string()),
                    InputIdentifier('a'),
                )),
            ),
        ];

        for (test_id, (input, expected_res)) in inputs.into_iter().enumerate() {
            let parse_result = parse(&input);
            assert_eq!(
                (test_id, Ok(Definition(vec![Directive(expected_res)]))),
                (test_id, parse_result)
            )
        }
    }

    #[test]
    fn should_parse_multiline_definition() {
        let (input, expected) = (
            "DEFINE src AS and;
DEFINE dest AS not;
LINK src -> a OF dest;",
            vec![
                Directive(DirectiveItem::GateDef(GateDef::new(
                    GateIdentifier::try_new_unchecked("src".to_string()),
                    GateTy::And,
                ))),
                Directive(DirectiveItem::GateDef(GateDef::new(
                    GateIdentifier::try_new_unchecked("dest".to_string()),
                    GateTy::Not,
                ))),
                Directive(DirectiveItem::LinkDef(LinkDef::new(
                    GateIdentifier::try_new_unchecked("src".to_string()),
                    GateIdentifier::try_new_unchecked("dest".to_string()),
                    InputIdentifier('a'),
                ))),
            ],
        );

        let parse_result = parse(&input);
        assert_eq!(Ok(Definition(expected)), parse_result)
    }

    #[test]
    fn should_fail_to_parse_gate_definition_with_invalid_id() {
        let input = "DEFINE InVaLiD AS and;";

        let parse_result = parse(&input);
        assert!(parse_result.is_err())
    }

    #[test]
    fn should_fail_to_parse_link_definition_with_invalid_input_id() {
        let input = "LINK src -> 1 OF dest;";

        let parse_result = parse(&input);
        assert!(parse_result.is_err())
    }
}
