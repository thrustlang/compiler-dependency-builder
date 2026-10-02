/*

    Copyright (C) 2026  Stevens Benavides

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

*/

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
