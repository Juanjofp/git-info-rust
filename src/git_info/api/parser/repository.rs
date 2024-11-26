use super::{constants, ApiError, GitRepositories, GitRepository, Parser};

impl Parser {
    pub fn repositories(body: Option<&String>, url: &str) -> Result<GitRepositories, ApiError> {
        let json_array = Parser::get_body_as_json_array(body, url)?;

        let repos = json_array
            .iter()
            .fold(GitRepositories::new(), |repsoitories, repo_str| {
                let Ok(repo) = Parser::repository_from_value(repo_str, url) else {
                    return repsoitories;
                };

                repsoitories.add(repo);

                repsoitories
            });

        Ok(repos)
    }

    pub fn repository(body: Option<&String>, url: &str) -> Result<GitRepository, ApiError> {
        let json = Parser::get_body_as_json(body, url)?;

        Parser::repository_from_value(&json, url)
    }

    fn repository_from_value(
        value: &serde_json::Value,
        url: &str,
    ) -> Result<GitRepository, ApiError> {
        let url = Some(url.to_string());

        let Some(name) = value.get(constants::fields::NAME) else {
            return Err(ApiError::field_not_found(constants::fields::NAME, url));
        };

        let Some(name) = name.as_str() else {
            return Err(ApiError::field_not_found(constants::fields::NAME, url));
        };

        let description = value
            .get(constants::fields::DESCRIPTION)
            .and_then(|d| d.as_str())
            .unwrap_or("");

        Ok(GitRepository::new(
            name.to_string(),
            description.to_string(),
        ))
    }
}
