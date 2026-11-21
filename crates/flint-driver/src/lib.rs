#[derive(Debug)]
pub struct MainError;

pub type MainResult = Result<(), MainError>;

/// # Errors
/// If the app returned an error.
pub const fn main() -> MainResult {
    Ok(())
}
