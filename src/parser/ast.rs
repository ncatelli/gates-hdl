#[derive(Debug, PartialEq)]
pub struct Definition(Vec<Directive>);

#[derive(Debug, PartialEq)]
pub struct Directive(pub DirectiveItem);

#[derive(Debug, PartialEq)]
pub enum DirectiveItem {
    GateDef(GateDef),
    LinkDef(LinkDef),
}

#[derive(Debug, PartialEq)]
pub struct GateDef {
    identifier: GateIdentifier,
    ty: GateTy,
}

impl GateDef {
    pub fn new(identifier: GateIdentifier, ty: GateTy) -> Self {
        Self { identifier, ty }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GateTy {
    Not,
    And,
    Or,
    Xor,
    Nand,
    Nor,
}

impl From<GateTy> for &str {
    fn from(gt: GateTy) -> Self {
        match gt {
            GateTy::Not => "not",
            GateTy::And => "and",
            GateTy::Or => "or",
            GateTy::Xor => "xor",
            GateTy::Nand => "nand",
            GateTy::Nor => "nor",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct GateIdentifier(String);

impl GateIdentifier {
    pub fn try_new(s: String) -> Option<Self> {
        // c.is_ascii_lowercase().then(|| Self(c))
        None
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
    src: GateIdentifier,
    dest: GateIdentifier,
    input: InputIdentifier,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InputIdentifier(char);

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
