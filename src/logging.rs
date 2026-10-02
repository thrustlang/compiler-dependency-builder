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

#[derive(Debug, PartialEq)]
pub enum LoggingType {
    Error,
    Panic,
    Warning,
    Debug,
}

#[derive(Debug)]
pub enum OutputIn {
    Stdout,
    Stderr,
}

impl std::fmt::Display for LoggingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggingType::Error => write!(f, "ERROR"),
            LoggingType::Panic => write!(f, "PANIC"),
            LoggingType::Warning => write!(f, "WARNING"),
            LoggingType::Debug => write!(f, "DEBUG"),
        }
    }
}

impl LoggingType {
    #[inline]
    pub fn is_panic(&self) -> bool {
        matches!(self, LoggingType::Panic)
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        matches!(self, LoggingType::Error)
    }
}

#[inline]
pub fn write(output_in: OutputIn, text: &str) {
    match output_in {
        OutputIn::Stdout => {
            let _ = std::io::Write::write_all(&mut std::io::stdout(), text.as_bytes());
        }

        OutputIn::Stderr => {
            let _ = std::io::Write::write_all(&mut std::io::stderr(), text.as_bytes());
        }
    };
}

pub fn log(ltype: LoggingType, msg: &str) {
    if ltype.is_panic() {
        let _ = std::io::Write::write_all(
            &mut std::io::stderr(),
            format!("{} {}", ltype, msg).as_bytes(),
        );

        std::process::exit(1);
    }

    if ltype.is_err() {
        let _ = std::io::Write::write_all(
            &mut std::io::stderr(),
            format!("{} {}", ltype, msg).as_bytes(),
        );

        return;
    }

    let _ = std::io::Write::write_all(
        &mut std::io::stdout(),
        format!("{} {}", ltype, msg).as_bytes(),
    );
}
