//! The command `just generate`, `just contract-check`, `just reads` and `just take` run.

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match lemonfiber_sdk_generate::run(&arguments, Path::new(".")) {
        Ok(said) => {
            println!("{said}");
            ExitCode::SUCCESS
        }
        Err(refused) => {
            eprintln!("{refused}");
            ExitCode::FAILURE
        }
    }
}
