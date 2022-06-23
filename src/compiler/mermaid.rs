use super::Compile;
use crate::type_check::Gate;

const PREFIX: &str = "graph LR";
const LEGEND: &str = "subgraph Legend
    not{{not}}
    and(and)
    or((or))
    xor>xor]
    nand[/nand/]
    nor[\\nor\\]
end";

pub fn compile(ctx: crate::type_check::BuildContext) -> Result<String, String> {
    MermaidGraph::compile(ctx).map(|mermaid| mermaid.to_string())
}

fn gate_to_node_shape<S: AsRef<str>>(id: S, gate: &Gate) -> String {
    use crate::parser::ast::GateTy;

    match gate.as_type() {
        GateTy::Not => format!("{{{}}}", id.as_ref()),
        GateTy::And => format!("({})", id.as_ref()),
        GateTy::Or => format!("(({}))", id.as_ref()),
        GateTy::Xor => format!(">{}]", id.as_ref()),
        GateTy::Nand => format!("[/{}/]", id.as_ref()),
        GateTy::Nor => format!("[\\{}\\]", id.as_ref()),
    }
}

pub struct MermaidGraph {
    repr: String,
}

impl ToString for MermaidGraph {
    fn to_string(&self) -> String {
        self.repr.clone()
    }
}

impl Compile for MermaidGraph {
    type Input = crate::type_check::BuildContext;

    fn compile(input: Self::Input) -> Result<Self, String> {
        let (mappings, gates, links) = input.into_raw_parts();
        let directives: String = ["subgraph chart\n".to_string()]
            .into_iter()
            .chain(
                mappings
                    .keys()
                    .zip(gates.iter())
                    .zip(links.into_iter())
                    .flat_map(|((id, ty), links)| {
                        let gate_def = format!("{}{}\n", id, gate_to_node_shape(id, ty));
                        let link_defs = links
                            .iter()
                            .map(|ln| format!("{} -- {} --> {}\n", &ln.src, ln.input, &ln.dest));
                        [gate_def].into_iter().chain(link_defs).collect::<Vec<_>>()
                    }),
            )
            .chain(["end\n".to_string()].into_iter())
            .collect();

        let graph = vec![PREFIX.to_string(), LEGEND.to_string(), directives].join("\n");

        Ok(MermaidGraph { repr: graph })
    }
}
