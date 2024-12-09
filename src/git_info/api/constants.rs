pub mod paths {
    pub const USER: &str = "/users";
    pub const ME: &str = "/user";
    pub const REPOS: &str = "/repos";
    pub const EVENTS: &str = "/events";
    pub const COMMITS: &str = "/commits";
}

pub mod fields {
    pub const ID: &str = "id";
    pub const LOGIN: &str = "login";
    pub const EMAIL: &str = "email";
    pub const AVATAR_URL: &str = "avatar_url";
    pub const DEFAULT_AVATAR: &str = "https://avatars.githubusercontent.com/u/446496?v=4";
    pub const NAME: &str = "name";
    pub const PUB_REPOS: &str = "public_repos";
    pub const PUB_GISTS: &str = "public_gists";
    pub const FOLLOWERS: &str = "followers";
    pub const FOLLOWING: &str = "following";
    pub const PRIV_REPOS: &str = "total_private_repos";
    pub const PRIV_GIST: &str = "private_gists";
    pub const OWNED_PRIV_REPOS: &str = "owned_private_repos";
    pub const DISK_USAGE: &str = "disk_usage";
    pub const COLLABORATORS: &str = "collaborators";
    pub const DESCRIPTION: &str = "description";
    pub const TYPE: &str = "type";
    pub const CREATED_AT: &str = "created_at";
    pub const UPDATED_AT: &str = "updated_at";
    pub const COMMIT: &str = "commit";
    pub const COMMITTER: &str = "committer";
    pub const AUTHOR: &str = "author";
    pub const DATE: &str = "date";
    pub const MESSAGE: &str = "message";
}
