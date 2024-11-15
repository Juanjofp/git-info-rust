#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub avatar: String,
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
