#[derive(Debug, PartialEq)]
pub enum GitError {
    InvalidGitFolder,
    CannotOpenFolder,
    NoGitFolder,
}
