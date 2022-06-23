pub mod compiler;
mod parser;
mod type_check;

pub use compiler::Compile;
pub use parser::parse;
pub use type_check::check;
