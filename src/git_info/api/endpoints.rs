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
        let path = format!("{}/{}", constants::paths::USER, username);

        Endpoints::prepare_url(&path)
    }

    pub fn repositories(username: &str) -> String {
        let path = format!(
            "{}/{}{}",
            constants::paths::USER,
            username,
            constants::paths::REPOS
        );

        Endpoints::prepare_url(&path)
    }

    pub fn repository(username: &str, repo: &str) -> String {
      // GET /repos/OWNER/REPO
        let path = format!(
            "{}/{}/{}",
            constants::paths::REPOS,
            username,
            repo
        );

        Endpoints::prepare_url(&path)
    }

    pub fn events(username: &str) -> String {
        let path = format!(
            "{}/{}{}",
            constants::paths::USER,
            username,
            constants::paths::EVENTS
        );

        Endpoints::prepare_url(&path)
    }

    pub fn commits(username: &str, repo: &str) -> String {
      // GET /repos/{owner}/{repo}/commits
        let path = format!(
            "{}/{}/{}{}",
            constants::paths::REPOS,
            username,
            repo,
            constants::paths::COMMITS
        );

        Endpoints::prepare_url(&path)
    }
}
