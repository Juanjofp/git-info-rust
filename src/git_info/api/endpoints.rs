use super::constants;

pub struct Endpoints;

const HOST: &str = "https://api.github.com";

impl Endpoints {
    fn prepare_url(path: &str) -> String {
        format!("{}{}", HOST, path)
    }

    pub fn me() -> String {
        Endpoints::prepare_url(constants::paths::ME)
    }

    pub fn user(username: &str) -> String {
        let path = format!("{}{}", constants::paths::USER, username);

        Endpoints::prepare_url(&path)
    }

    pub fn repositories(username: &str) -> String {
        let path = format!(
            "{}{}{}",
            constants::paths::USER,
            username,
            constants::paths::REPOS
        );

        Endpoints::prepare_url(&path)
    }
}
