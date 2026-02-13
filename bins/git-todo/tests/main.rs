use crate::git::GitRepo;

mod git;

#[test]
fn test_add_and_list() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("implement secret feature 1"));
    assert!(str.contains("implement secret feature 2"));
    assert!(str.contains("implement secret feature 3"));
}

#[test]
fn test_clear_all() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    repo.run_todo(&["clear"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert_eq!(str, "");
}

#[test]
fn test_clear_complete_only() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    repo.run_todo(&["complete", "0", "1"]);
    repo.run_todo(&["clear", "--done"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("implement secret feature 3"));
}

#[test]
fn test_complete() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    repo.run_todo(&["complete", "1", "2"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 0:"));
    assert!(str.contains("[x] 1:"));
    assert!(str.contains("[x] 2:"));

    repo.run_todo(&["complete", "0", "1"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[x] 0:"));
    assert!(str.contains("[ ] 1:"));
    assert!(str.contains("[x] 2:"));
}

#[test]
fn test_complete_undone() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    repo.run_todo(&["complete", "0"]);
    let output = repo.run_todo(&["list", "--undone"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("[ ] 1:"));
    assert!(str.contains("[ ] 2:"));
}

#[test]
fn test_delete() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    repo.run_todo(&["add", "implement secret feature 3"]);
    repo.run_todo(&["delete", "0", "1"]);
    let output = repo.run_todo(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(!str.contains("implement secret feature 1"));
    assert!(!str.contains("implement secret feature 2"));
    assert!(str.contains("implement secret feature 3"));
}

#[test]
fn test_check_status_code() {
    let repo = GitRepo::new();
    repo.run_todo(&["add", "implement secret feature 1"]);
    repo.run_todo(&["add", "implement secret feature 2"]);
    assert!(!repo.run_todo(&["check", "--quiet"]).status.success());

    repo.run_todo(&["complete", "0"]);
    assert!(!repo.run_todo(&["check", "--quiet"]).status.success());

    repo.run_todo(&["complete", "1"]);
    assert!(repo.run_todo(&["check", "--quiet"]).status.success());
}
