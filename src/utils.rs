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

use std::{path::PathBuf, process::Command};

use crate::logging::{self, LoggingType};

pub fn get_suitable_default_c_compiler() -> &'static str {
    match std::env::consts::OS {
        "linux" => {
            if is_clang_c_available() {
                "clang"
            } else {
                "gcc"
            }
        }

        "windows" => "cl",

        "macos" => {
            if is_clang_c_available() {
                "clang"
            } else {
                "gcc"
            }
        }

        _ => "clang",
    }
}

pub fn get_suitable_default_cpp_compiler() -> &'static str {
    match std::env::consts::OS {
        "linux" => {
            if is_clang_cpp_available() {
                "clang++"
            } else {
                "g++"
            }
        }

        "windows" => "cl",

        "macos" => {
            if is_clang_cpp_available() {
                "clang++"
            } else {
                "g++"
            }
        }

        _ => "clang++",
    }
}

pub fn get_compiler_llvm_build_path() -> PathBuf {
    match std::env::consts::FAMILY {
        "unix" => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| {
            logging::log(LoggingType::Panic, "Missing $HOME environment variable.\n");
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/llvm/build"),

        "windows" => PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| {
            logging::log(
                LoggingType::Panic,
                "Missing $APPDATA environment variable.\n",
            );
            std::process::exit(1);
        }))
        .join(".thrustlang/backends/llvm/build"),

        _ => {
            logging::log(
                LoggingType::Panic,
                "Unsupported operating system for build the dependencies required to build the Thrust Compiler.",
            );

            std::process::exit(1);
        }
    }
}

#[inline]
pub fn is_tar_available() -> bool {
    Command::new("tar").arg("--version").output().is_ok()
}

#[inline]
pub fn is_cmake_available() -> bool {
    Command::new("cmake").arg("--version").output().is_ok()
}

#[inline]
pub fn is_ninja_available() -> bool {
    Command::new("ninja").arg("--version").output().is_ok()
}

#[inline]
pub fn is_clang_cpp_available() -> bool {
    Command::new("clang++").arg("--version").output().is_ok()
}

#[inline]
pub fn is_clang_c_available() -> bool {
    Command::new("clang").arg("--version").output().is_ok()
}

#[inline]
pub fn restore_llvm_build_path() {
    let _ = std::fs::remove_dir(self::get_compiler_llvm_build_path());
    let _ = std::fs::create_dir_all(self::get_compiler_llvm_build_path());
}
