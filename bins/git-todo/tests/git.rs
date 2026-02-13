use std::fs::File;
use std::process::{Command, Output};
use tempdir::TempDir;

/// Represents a git repository.
pub struct GitRepo {
    /// The path to the repository.
    path: TempDir,
}

impl GitRepo {
    /// Creates a new git repository.
    pub fn new() -> Self {
        let dir = TempDir::new("temp_").expect("Failed to create temp dir");
        File::create(&dir.path().join("test.txt")).expect("Failed to create test file");
        let repo = Self { path: dir };
        repo.run_git(&["init"]);
        repo.run_git(&["config", "user.email", "test@test.com"]);
        repo.run_git(&["config", "user.name", "Test User"]);
        repo.run_git(&["add", "."]);
        repo.run_git(&["commit", "-m", "feat: initial commit"]);
        repo
    }

    /// Runs a git command in the repository.
    pub fn run_git(&self, args: &[&str]) -> Output {
        self.run_cmd("git", args)
    }

    /// Runs the TODO command in the repository.
    pub fn run_todo(&self, args: &[&str]) -> Output {
        self.run_cmd(env!("CARGO_BIN_EXE_git-todo"), args)
    }

    /// Runs a command.
    fn run_cmd(&self, program: &str, args: &[&str]) -> Output {
        Command::new(program)
            .current_dir(&self.path)
            .args(args)
            .output()
            .expect("Failed to run command")
    }
}
