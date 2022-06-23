#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

pub mod compiler;
mod parser;
mod type_check;

pub use compiler::Compile;
pub use parser::parse;
pub use type_check::check;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn compile_compose(src: &str) -> Result<String, String> {
    compile_compose_inner(src)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn compile_compose<S: AsRef<str>>(src: S) -> Result<String, String> {
    compile_compose_inner(src)
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn generate_mermaid(src: &str) -> Result<String, String> {
    generate_mermaid_inner(src)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_mermaid<S: AsRef<str>>(src: S) -> Result<String, String> {
    generate_mermaid_inner(src)
}

fn compile_compose_inner<S: AsRef<str>>(src: S) -> Result<String, String> {
    let build_ctx = parse(src.as_ref())
        .map_err(|e| format!("{:?}", e))
        .and_then(|ast| check(ast).map_err(|e| format!("{:?}", e)))?;

    compiler::compile(build_ctx)
}

fn generate_mermaid_inner<S: AsRef<str>>(src: S) -> Result<String, String> {
    let build_ctx = parse(src.as_ref())
        .map_err(|e| format!("{:?}", e))
        .and_then(|ast| check(ast).map_err(|e| format!("{:?}", e)))?;

    compiler::mermaid::compile(build_ctx)
}
