pub mod compose;
pub mod mermaid;

pub trait Compile
where
    Self: Sized,
{
    type Input;

    fn compile(input: Self::Input) -> Result<Self, String>;
}

pub use compose::compile;
