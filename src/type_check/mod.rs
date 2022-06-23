use crate::parser::ast;
use std::collections::HashMap;

mod ordered_hash_map;

use ordered_hash_map::OrderedHashMap;

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

    pub fn as_str(&self) -> &'static str {
        self.ty.into()
    }
}

impl AsRef<str> for Gate {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Link {
    pub src: String,
    pub dest: String,
    pub input: char,
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
    mappings: OrderedHashMap<String, usize>,
    gates: Vec<Gate>,
    links: Vec<Vec<Link>>,
}

impl BuildContext {
    pub fn new(mappings: HashMap<String, usize>, gates: Vec<Gate>, links: Vec<Vec<Link>>) -> Self {
        let mappings = mappings
            .into_iter()
            .fold(OrderedHashMap::default(), |mut acc, (k, v)| {
                acc.insert(k, v);
                acc
            });

        Self {
            mappings,
            gates,
            links,
        }
    }

    pub(crate) fn into_raw_parts(
        self,
    ) -> (OrderedHashMap<String, usize>, Vec<Gate>, Vec<Vec<Link>>) {
        (self.mappings, self.gates, self.links)
    }
}

pub fn check(def: ast::Definition) -> Result<BuildContext, String> {
    use ast::{DirectiveItem, GateDef, LinkDef};

    let mut mappings: HashMap<String, usize> = HashMap::default();
    let mut gates: Vec<Gate> = vec![];
    let mut links: Vec<Vec<Link>> = vec![];
    let directives = def.as_ref();

    for (line, di) in directives.iter().map(|d| &d.0).enumerate() {
        match di {
            DirectiveItem::GateDef(GateDef { identifier, ty }) => {
                let next_idx = gates.len();
                if !mappings.contains_key(identifier.as_ref()) {
                    mappings.insert(identifier.to_string(), next_idx);
                    gates.push(Gate::new(*ty));
                    links.push(vec![])
                } else {
                    return Err(format!(
                        "{} gate id ({}) already defined",
                        LineNumber(line),
                        identifier.as_ref()
                    ));
                };
            }
            DirectiveItem::LinkDef(LinkDef { src, dest, input }) => {
                let &src_idx = mappings
                    .get(src.as_ref())
                    .ok_or_else(|| format!("{} source gate ({}) undefined", line, src.as_ref()))?;

                let &dest_idx = mappings.get(dest.as_ref()).ok_or_else(|| {
                    format!("{} destination gate ({}) undefined", line, dest.as_ref())
                })?;
                let dest_ty = gates.get(dest_idx).ok_or_else(|| {
                    format!("{} destination gate ({}) undefined", line, dest.as_ref())
                })?;

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
                    links
                        .get_mut(src_idx)
                        .map(|outbound_links| {
                            outbound_links.push(Link::new(
                                src.as_ref().to_string(),
                                dest.to_string(),
                                input.as_char(),
                            ));
                        })
                        .ok_or_else(|| {
                            format!("{} no links defined for src gate ({})", line, src.as_ref())
                        })?;
                }
            }
        }
    }

    Ok(BuildContext::new(mappings, gates, links))
}
