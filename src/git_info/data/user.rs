use std::rc::Rc;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitUser {
    pub id: String,
    pub login: String,
    pub email: Option<String>,
    pub avatar: String,
}

impl GitUser {
    pub fn new(id: String, login: String, email: Option<String>, avatar: String) -> Self {
        GitUser {
            login,
            id,
            email,
            avatar,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitUserInfo {
    pub user: Rc<GitUser>,

    pub name: Option<String>,

    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u32,
    pub following: u32,

    pub private_repos: u32,
    pub private_gists: u32,
    pub owned_private_repos: u32,

    pub collaborators: u32,

    pub disk_usage: u32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GitUserInfo {
    pub fn new(
        user: Rc<GitUser>,

        name: Option<String>,

        public_repos: u32,
        public_gists: u32,
        followers: u32,
        following: u32,

        private_repos: u32,
        private_gists: u32,
        owned_private_repos: u32,

        collaborators: u32,

        disk_usage: u32,

        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        GitUserInfo {
            user,
            name,
            public_repos,
            public_gists,
            followers,
            following,
            private_repos,
            private_gists,
            owned_private_repos,
            collaborators,
            disk_usage,
            created_at,
            updated_at,
        }
    }
}
