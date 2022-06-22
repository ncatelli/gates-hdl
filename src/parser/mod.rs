use parcel::parsers::character::{any_character, expect_character, expect_str};
use parcel::prelude::v1::*;

mod ast;

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
    directive_item().map(ast::Directive)
}

fn directive_item<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::DirectiveItem> {
    parcel::or(gate_def().map(ast::DirectiveItem::GateDef), || {
        link_def().map(ast::DirectiveItem::LinkDef)
    })
}

fn gate_def<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::GateDef> {
    parcel::join(
        parcel::right(parcel::join(
            whitespace_wrapped(expect_str("DEFINE")),
            whitespace_wrapped(gate_identifier()),
        )),
        parcel::right(parcel::join(
            whitespace_wrapped(expect_str("AS")),
            whitespace_wrapped(gate_ty()),
        )),
    )
    .map(|(id, ty)| ast::GateDef::new(id, ty))
}

fn gate_ty<'a>() -> impl Parser<'a, &'a [(usize, char)], ast::GateTy> {
    whitespace_wrapped(parcel::one_of(vec![
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
            whitespace_wrapped(expect_str("LINK")),
            whitespace_wrapped(gate_identifier()),
        )),
        parcel::join(
            parcel::right(parcel::join(
                whitespace_wrapped(expect_str("->")),
                whitespace_wrapped(input_identifier()),
            )),
            parcel::right(parcel::join(
                whitespace_wrapped(expect_str("OF")),
                whitespace_wrapped(gate_identifier()),
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

fn whitespace_wrapped<'a, P, B>(parser: P) -> impl Parser<'a, &'a [(usize, char)], B>
where
    B: 'a,
    P: Parser<'a, &'a [(usize, char)], B> + 'a,
{
    use parcel::parsers::character::whitespace;

    parcel::right(parcel::join(
        parcel::zero_or_more(whitespace()),
        parcel::left(parcel::join(parser, parcel::zero_or_more(whitespace()))),
    ))
}
