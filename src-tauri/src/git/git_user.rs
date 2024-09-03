#[derive(Clone)]
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
        let name = "name".to_string();
        let email = "email".to_string();
        let git_user = GitUser::new(name.clone(), email.clone());
        assert_eq!(git_user.name, name);
        assert_eq!(git_user.email, email);
    }
}
