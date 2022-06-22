pub mod compose;

pub trait Compile
where
    Self: Sized,
{
    type Input;

    fn compile(input: Self::Input) -> Result<Self, String>;
}
