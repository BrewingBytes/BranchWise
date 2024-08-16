#[derive(Debug, PartialEq)]
pub enum GitError {
    CannotOpenFolder,
    NoGitFolder,
}
