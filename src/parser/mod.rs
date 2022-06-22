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
    let _input: Vec<(usize, char)> = input.as_ref().chars().enumerate().collect();
    todo!()
}
