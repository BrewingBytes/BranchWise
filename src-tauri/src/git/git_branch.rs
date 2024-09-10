use serde::{Serialize, Deserialize};

use super::git_commit::GitCommit;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitBranch {
    name: String,
    commit: GitCommit,
}

impl GitBranch {
    pub fn new(name: String, commit: GitCommit) -> GitBranch {
        GitBranch { name, commit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_branch_new() {
        let branch = GitBranch::new("test".to_string(), 
    GitCommit {
        hash: "test".to_string(),
    });
        assert_eq!(branch.name, "test");
        assert_eq!(branch.commit, "test");
    }
}
