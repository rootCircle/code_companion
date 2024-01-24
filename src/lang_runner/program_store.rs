use crate::lang_runner::language::Language;
use crate::utils::file_utils;
use std::path::Path;
use std::process::exit;

const FILE_NOT_FOUND_EXIT_CODE: i32 = 6;

#[derive(Debug)]
pub struct ProgramStore {
    correct_file: Language,
    test_file: Language,
    correct_file_bin_path: String,
    test_file_bin_path: String,
}

impl ProgramStore {
    pub fn new(correct_file: &Path, test_file: &Path, do_force_compile: bool) -> Self {
        if !Self::exists(correct_file, test_file) {
            eprintln!("[ERROR] File(s) doesn't exists\nQuitting.....");
            exit(FILE_NOT_FOUND_EXIT_CODE);
        }

        let mut program_store = ProgramStore {
            correct_file: Language::new(correct_file, do_force_compile),
            test_file: Language::new(test_file, do_force_compile),
            correct_file_bin_path: "".to_string(),
            test_file_bin_path: "".to_string(),
        };

        program_store.correct_file_bin_path = program_store.correct_file.warmup_precompile().unwrap();
        program_store.test_file_bin_path = program_store.test_file.warmup_precompile().unwrap();

        program_store

    }

    fn exists(correct_file: &Path, test_file: &Path) -> bool {
        correct_file.exists() && test_file.exists()
    }

    pub fn run_code(&self, stdin_content: &str) -> Result<(bool, String, String), &str> {
        let correct_file = self.correct_file.run_program_code(&self.correct_file_bin_path, stdin_content);
        let test_file = self.test_file.run_program_code(&self.test_file_bin_path, stdin_content);

        match correct_file {
            Ok(correct_output) => match test_file {
                Ok(test_output) => Ok((
                    file_utils::string_diff(&correct_output, &test_output),
                    correct_output,
                    test_output,
                )),
                Err(err) => {
                    eprintln!("Failed to run test file!\n{err}");
                    Err("Error running Test File")
                }
            },
            Err(err) => {
                eprintln!("Failed to run source file!\n{err}");
                Err("Error running Source File")
            }
        }
    }
}