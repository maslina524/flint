use flint_fmt::Renderable;

pub trait Module {
    type Output: Renderable + 'static;
    type Error: Renderable + 'static;

    /// # Errors
    /// If failed to compute module.
    fn compute() -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use flint_fmt::Renderable;

    use crate::Module;

    #[test]
    fn example_module() {
        pub struct ExampleOut(i32);

        impl Renderable for ExampleOut {}
        impl fmt::Display for ExampleOut {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Example: {}", self.0)
            }
        }

        pub struct Example;
        impl Module for Example {
            type Output = ExampleOut;
            type Error = !;

            fn compute() -> Result<Self::Output, Self::Error> {
                Ok(ExampleOut(42))
            }
        }

        let example = Example::compute();
        let out = example.unwrap_or_else(|_| unreachable!());
        assert_eq!(format!("{out}"), "Example: 42");
    }
}
