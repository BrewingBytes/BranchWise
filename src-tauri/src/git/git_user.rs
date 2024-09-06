#[derive(Debug, Clone, PartialEq)]
pub struct GitUser {
    pub name: String,
    pub email: String,
}

impl GitUser {
    pub fn new(name: String, email: String) -> GitUser {
        GitUser { name, email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let git_user = GitUser::new("name".to_string(), "email".to_string());
        assert_eq!(git_user.name, "name".to_string());
        assert_eq!(git_user.email, "email".to_string());
    }
}
