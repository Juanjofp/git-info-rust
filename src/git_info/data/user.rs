#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitUser {
    name: String,
    email: String,
    avatar: String,
}

impl GitUser {
    pub fn new(name: String, email: String, avatar: String) -> Self {
        GitUser {
            name,
            email,
            avatar,
        }
    }
}
