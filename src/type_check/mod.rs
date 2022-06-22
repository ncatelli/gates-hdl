use crate::parser::ast;
use std::collections::{HashMap, HashSet};

pub struct Gate {
    ty: ast::GateTy,
}

impl Gate {
    fn new(ty: ast::GateTy) -> Self {
        Self { ty }
    }

    fn inputs(&self) -> u32 {
        match self.ty {
            ast::GateTy::Not => 1,
            ast::GateTy::And
            | ast::GateTy::Or
            | ast::GateTy::Xor
            | ast::GateTy::Nand
            | ast::GateTy::Nor => 2,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Link {
    src: String,
    dest: String,
    input: char,
}

impl Link {
    fn new(src: String, dest: String, input: char) -> Self {
        Self { src, dest, input }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct LineNumber(usize);

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}:", self.0)
    }
}

#[derive(Default)]
pub struct BuildContext {
    gates: HashMap<String, Gate>,
    links: HashSet<Link>,
}

impl BuildContext {
    pub fn new(gates: HashMap<String, Gate>, links: HashSet<Link>) -> Self {
        Self { gates, links }
    }
}

pub(crate) fn check(def: ast::Definition) -> Result<BuildContext, String> {
    use ast::{DirectiveItem, GateDef, LinkDef};

    let mut gates: HashMap<String, Gate> = HashMap::default();
    let mut links: HashSet<Link> = HashSet::default();
    let directives = def.as_ref();

    for (line, di) in directives.iter().map(|d| &d.0).enumerate() {
        if let DirectiveItem::GateDef(GateDef { identifier, ty }) = di {
            gates
                .insert(identifier.to_string(), Gate::new(*ty))
                .ok_or_else(|| {
                    format!(
                        "{} gate id ({}) already defined",
                        LineNumber(line),
                        identifier.as_ref()
                    )
                })?;
        }
    }

    for (line, di) in directives.iter().map(|d| &d.0).enumerate() {
        if let DirectiveItem::LinkDef(LinkDef { src, dest, input }) = di {
            let (src_id, _) = gates.get_key_value(src.as_ref()).ok_or_else(|| {
                format!("{} source gate ({}) undefined defined", line, src.as_ref())
            })?;

            let dest_ty = gates
                .get(dest.as_ref())
                .ok_or_else(|| format!("{} destination gate ({}) defined", line, src.as_ref()))?;

            let min = 'a' as u32;
            let max = min + dest_ty.inputs();
            let input_offset = input.as_char() as u32;
            if !(input_offset >= min && input_offset < max) {
                return Err(format!(
                    "{} input exceeds maxinum allowable input of {} for gate ({})",
                    line,
                    dest_ty.inputs(),
                    dest.as_ref()
                ));
            } else {
                links.insert(Link::new(
                    src_id.to_string(),
                    dest.to_string(),
                    input.as_char(),
                ));
            }
        }
    }

    Ok(BuildContext::new(gates, links))
}
