use crate::logging::LoggingType;
use crate::options::BuildOptions;
use crate::{gcc, llvm, logging, utils};

#[derive(Debug)]
pub struct CompilerBuilderDependencies<'a> {
    options: &'a BuildOptions,
}

impl<'a> CompilerBuilderDependencies<'a> {
    #[inline]
    pub fn new(options: &'a BuildOptions) -> Self {
        Self { options }
    }
}

impl<'a> CompilerBuilderDependencies<'a> {
    pub fn build(&self) {
        let options: &BuildOptions = self.get_build_options();

        self.build_llvm_project()
            .unwrap_or_else(|error| logging::log(LoggingType::Panic, &error));

        logging::write(logging::OutputIn::Stdout, "LLVM installed.\n\n");

        if options.get_build_gcc_backend() {
            self.build_gcc_project()
                .unwrap_or_else(|error| logging::log(LoggingType::Panic, &error));

            logging::write(logging::OutputIn::Stdout, "GCC installed.\n\n");
        }
    }
}

impl CompilerBuilderDependencies<'_> {
    fn build_llvm_project(&self) -> Result<(), String> {
        let llvm_build: &llvm::LLVMBuild = self.get_build_options().get_llvm_build();

        if utils::get_compiler_llvm_build_path().exists() {
            logging::write(
                logging::OutputIn::Stdout,
                "LLVM was installed before, skipping...\n",
            );

            return Ok(());
        }

        utils::restore_llvm_build_path();

        logging::write(logging::OutputIn::Stdout, "Downloading LLVM source...\n");

        let llvm_downloaded: std::path::PathBuf = llvm::download_llvm(llvm_build)?;
        let llvm_source: std::path::PathBuf = llvm::decompress_llvm(llvm_build, &llvm_downloaded)?;

        logging::write(logging::OutputIn::Stdout, "Building LLVM from source...\n");

        llvm::prepare_build_directory(&llvm_source)?;
        llvm::build_and_install(llvm_build, llvm_downloaded, llvm_source)?;

        Ok(())
    }
}

impl CompilerBuilderDependencies<'_> {
    fn build_gcc_project(&self) -> Result<(), String> {
        let gcc_build: &gcc::GCCBuild = self.get_build_options().get_gcc_build();

        logging::write(logging::OutputIn::Stdout, "Downloading GCC source...\n");

        let gcc_downloaded: std::path::PathBuf = gcc::download_gcc(gcc_build)?;
        let gcc_source: std::path::PathBuf = gcc::decompress_gcc(gcc_build, &gcc_downloaded)?;

        logging::write(logging::OutputIn::Stdout, "Building GCC from source...\n");

        gcc::prepare_build_directory(&gcc_source)?;
        gcc::build_and_install(gcc_build, gcc_downloaded, gcc_source)?;

        Ok(())
    }
}

impl CompilerBuilderDependencies<'_> {
    #[inline]
    pub fn get_build_options(&self) -> &BuildOptions {
        self.options
    }
}
