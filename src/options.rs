use crate::gcc::GCCBuild;
use crate::llvm::LLVMBuild;

#[derive(Debug)]
pub struct BuildOptions {
    llvm_build: LLVMBuild,
    gcc_build: GCCBuild,

    build_gcc_backend: bool,
    build_cbindgen: bool,
}

impl BuildOptions {
    #[inline]
    pub fn new() -> BuildOptions {
        BuildOptions {
            llvm_build: LLVMBuild::new(),
            gcc_build: GCCBuild::new(),

            build_gcc_backend: false,
            build_cbindgen: false,
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

    #[inline]
    pub fn get_build_cbindgen(&self) -> bool {
        self.build_cbindgen
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
