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

use crate::gcc::GCCBuild;
use crate::llvm::LLVMBuild;

#[derive(Debug)]
pub struct BuildOptions {
    llvm_build: LLVMBuild,
    gcc_build: GCCBuild,

    build_gcc_backend: bool,
}

impl BuildOptions {
    #[inline]
    pub fn new() -> BuildOptions {
        BuildOptions {
            llvm_build: LLVMBuild::new(),
            gcc_build: GCCBuild::new(),

            build_gcc_backend: false,
        }
    }
}

impl BuildOptions {
    #[inline]
    pub fn set_build_gcc_backend(&mut self, build_gcc_backend: bool) {
        self.build_gcc_backend = build_gcc_backend;
    }
}

impl BuildOptions {
    #[inline]
    pub fn get_llvm_build(&self) -> &LLVMBuild {
        &self.llvm_build
    }

    #[inline]
    pub fn get_gcc_build(&self) -> &GCCBuild {
        &self.gcc_build
    }
}

impl BuildOptions {
    #[inline]
    pub fn get_build_gcc_backend(&self) -> bool {
        self.build_gcc_backend
    }
}

impl BuildOptions {
    #[inline]
    pub fn get_mut_llvm_build(&mut self) -> &mut LLVMBuild {
        &mut self.llvm_build
    }

    #[inline]
    pub fn get_mut_gcc_build(&mut self) -> &mut GCCBuild {
        &mut self.gcc_build
    }
}
