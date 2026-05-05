use crate::builder::CompilerBuilderDependencies;
use crate::cli::CommandLine;

mod builder;
mod cli;
mod constants;
mod gcc;
mod help;
mod llvm;
mod logging;
mod options;
mod utils;

fn main() -> ! {
    unsafe { std::env::set_var("CARGO_TERM_VERBOSE", "true") };

    let command_line: CommandLine = CommandLine::parse(std::env::args().collect());
    let options: &options::BuildOptions = command_line.get_options();

    CompilerBuilderDependencies::new(options).build();

    std::process::exit(0)
}
