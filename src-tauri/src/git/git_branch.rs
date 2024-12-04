use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitBranch {
    pub name: String,
    commit: String,
}

impl GitBranch {
    pub fn new(name: String, commit: String) -> GitBranch {
        GitBranch { name, commit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_git_branch_new() {
        let branch = GitBranch::new("test".to_string(), "test".to_string());
        assert_eq!(branch.name, "test");
        assert_eq!(branch.commit, "test");
    }

    #[test]
    fn test_git_branch_serialize() {
        let branch = GitBranch::new("testBranch".to_string(), "commitHash".to_string());
        let serialized = serde_json::to_string(&branch).unwrap();
        let expected = r#"{"name":"testBranch","commit":"commitHash"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_git_branch_deserialize() {
        let json_data = r#"{"name":"testBranch","commit":"commitHash"}"#;
        let deserialized: GitBranch = serde_json::from_str(json_data).unwrap();
        let expected = GitBranch::new("testBranch".to_string(), "commitHash".to_string());
        assert_eq!(deserialized, expected);
    }
}
