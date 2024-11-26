pub mod paths {
    pub const USER: &str = "/users";
    pub const ME: &str = "/user";
    pub const REPOS: &str = "/repos";
    pub const EVENTS: &str = "/events";
    pub const COMMITS: &str = "/commits";
}

pub mod fields {
    pub const LOGIN: &str = "login";
    pub const EMAIL: &str = "email";
    pub const AVATAR_URL: &str = "avatar_url";
    pub const DEFAULT_AVATAR: &str = "https://avatars.githubusercontent.com/u/446496?v=4";
    pub const NAME: &str = "name";
    pub const DESCRIPTION: &str = "description";
    pub const TYPE: &str = "type";
    pub const CREATED_AT: &str = "created_at";
    pub const COMMIT: &str = "commit";
    pub const COMMITTER: &str = "committer";
    pub const AUTHOR: &str = "author";
    pub const DATE: &str = "date";
    pub const MESSAGE: &str = "message";
}
