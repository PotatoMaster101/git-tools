use crate::git::GitRepo;

mod git;

#[test]
fn test_add() {
    let repo = GitRepo::new();
    repo.run_user(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"]);
    repo.run_user(&["add", "user2", "user2@test.com", "-s", "ssh -i ~/.ssh/key2", "-k", "~/.ssh/key2.pub"]);
    repo.run_user(&["add", "user3", "user3@test.com", "-p", "personal"]);
    let output = repo.run_user(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user1@test.com"));
    assert!(str.contains("user2@test.com"));
    assert!(str.contains("SSH Command: ssh -i ~/.ssh/key1"));
    assert!(str.contains("Signing Key: ~/.ssh/key2.pub"));
    assert!(str.contains("SSH Command: ssh -i ~/.ssh/key2"));
    assert!(str.contains("personal"));
}

#[test]
fn test_delete() {
    let repo = GitRepo::new();
    repo.run_user(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"]);
    repo.run_user(&["add", "user2", "user2@test.com", "-s", "ssh -i ~/.ssh/key2", "-k", "~/.ssh/key2.pub"]);
    repo.run_user(&["delete", "user1"]);
    let output = repo.run_user(&["list"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(!str.contains("user1@test.com"));
    assert!(!str.contains("SSH Command: ssh -i ~/.ssh/key1"));
}

#[test]
fn test_use() {
    let repo = GitRepo::new();
    repo.run_user(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"]);
    repo.run_user(&["add", "user2", "user2@test.com", "-s", "ssh -i ~/.ssh/key2", "-k", "~/.ssh/key2.pub"]);
    repo.run_user(&["use", "user1"]);
    let output = repo.run_git(&["config", "--list", "--local"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user.name=user1"));
    assert!(str.contains("user.email=user1@test.com"));
    assert!(str.contains("core.sshcommand=ssh -i ~/.ssh/key1"));

    repo.run_user(&["use", "user2"]);
    let output = repo.run_git(&["config", "--list", "--local"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert!(str.contains("user.name=user2"));
    assert!(str.contains("user.email=user2@test.com"));
    assert!(str.contains("user.signingkey=~/.ssh/key2.pub"));
    assert!(str.contains("core.sshcommand=ssh -i ~/.ssh/key2"));
}

#[test]
fn test_export() {
    let repo = GitRepo::new();
    repo.run_user(&["add", "user1", "user1@test.com", "-s", "ssh -i ~/.ssh/key1"]);
    repo.run_user(&["add", "user2", "user2@test.com", "-s", "ssh -i ~/.ssh/key2", "-k", "~/.ssh/key2.pub"]);
    let output = repo.run_user(&["export"]);
    let str = String::from_utf8_lossy(output.stdout.as_slice());
    assert_ne!(str, "");
}
