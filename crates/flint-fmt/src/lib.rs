use std::fmt;

pub trait Renderable: fmt::Display {}

impl Renderable for ! {}

#[cfg(test)]
mod tests {
    use std::fmt;

    use crate::Renderable;

    #[test]
    fn render_for_human() {
        struct Human;

        impl Renderable for Human { }
        impl fmt::Display for Human {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Human output")
            }
        }

        let buf = format!("{Human}");
        assert_eq!(buf, "Human output");
    }
}