use std::process;
use flint_driver::MainResult;

/// # Call Stack
/// **flint** -> **flint_driver** -> build & run app -> detect modules -> print modules
fn main() -> MainResult {
    let result = flint_driver::main();

    match result {
        Ok(()) => process::exit(0),
        Err(_) => process::exit(101)
    }
}
