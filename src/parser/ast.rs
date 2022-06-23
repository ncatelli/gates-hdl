#[derive(Debug, PartialEq)]
pub struct Definition(pub Vec<Directive>);

impl AsRef<[Directive]> for Definition {
    fn as_ref(&self) -> &[Directive] {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Directive(pub DirectiveItem);

#[derive(Debug, PartialEq)]
pub enum DirectiveItem {
    GateDef(GateDef),
    LinkDef(LinkDef),
}

#[derive(Debug, PartialEq)]
pub struct GateDef {
    pub identifier: GateIdentifier,
    pub ty: GateTy,
}

impl GateDef {
    pub fn new(identifier: GateIdentifier, ty: GateTy) -> Self {
        Self { identifier, ty }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateTy {
    Not,
    And,
    Or,
    Xor,
    Nand,
    Nor,
}

impl GateTy {
    pub fn as_str(&self) -> &'static str {
        match self {
            GateTy::Not => "not",
            GateTy::And => "and",
            GateTy::Or => "or",
            GateTy::Xor => "xor",
            GateTy::Nand => "nand",
            GateTy::Nor => "nor",
        }
    }
}

impl From<GateTy> for &str {
    fn from(gt: GateTy) -> Self {
        gt.as_str()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct GateIdentifier(String);

impl GateIdentifier {
    pub fn try_new(s: String) -> Option<Self> {
        let head_is_valid = s
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false);
        let tail_is_valid = s.chars().skip(1).all(|c| c.is_ascii_lowercase());

        if head_is_valid && tail_is_valid {
            Some(GateIdentifier(s))
        } else {
            None
        }
    }

    pub fn try_new_unchecked(s: String) -> Self {
        Self(s)
    }
}

impl ToString for GateIdentifier {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl AsRef<str> for GateIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<GateIdentifier> for String {
    fn from(gi: GateIdentifier) -> Self {
        gi.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct LinkDef {
    pub src: GateIdentifier,
    pub dest: GateIdentifier,
    pub input: InputIdentifier,
}

impl LinkDef {
    pub fn new(src: GateIdentifier, dest: GateIdentifier, input: InputIdentifier) -> Self {
        Self { src, dest, input }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InputIdentifier(pub char);

impl InputIdentifier {
    pub fn try_new(c: char) -> Option<Self> {
        c.is_ascii_lowercase().then(|| Self(c))
    }

    pub fn try_new_unchecked(c: char) -> Self {
        Self(c)
    }

    pub fn as_char(&self) -> char {
        self.0
    }
}

impl From<InputIdentifier> for char {
    fn from(ii: InputIdentifier) -> Self {
        ii.as_char()
    }
}
