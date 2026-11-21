#[derive(Debug)]
pub struct MainError;

pub type MainResult = Result<(), MainError>;

pub fn main() -> MainResult {
    Ok(())
}