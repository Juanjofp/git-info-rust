use crate::git_info::data::GitUser;

use super::{constants, ApiError, GitRepositories, GitRepository};

pub struct Parser;

impl Parser {
    pub fn parse_git_repository(
        body: Option<&String>,
        url: &str,
    ) -> Result<GitRepositories, ApiError> {
        let json = Parser::get_body_as_json(body, url)?;

        let Some(json_array) = json.as_array() else {
            return Err(ApiError::invalid_json(Some(url.to_string())));
        };

        let repos = json_array
            .iter()
            .fold(GitRepositories::new(), |repsoitories, repo_str| {
                let Some(name) = repo_str.get(constants::fields::NAME) else {
                    return repsoitories;
                };

                let Some(name) = name.as_str() else {
                    return repsoitories;
                };

                let description = repo_str
                    .get(constants::fields::DESCRIPTION)
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                repsoitories.add_repository(GitRepository::new(
                    String::from(name),
                    String::from(description),
                ));

                repsoitories
            });

        Ok(repos)
    }

    pub fn parse_git_user(body: Option<&String>, url: &str) -> Result<GitUser, ApiError> {
        let json = Parser::get_body_as_json(body, url)?;

        let Some(user) = json[constants::fields::LOGIN].as_str() else {
            return Err(ApiError::field_not_found(constants::fields::LOGIN, None));
        };

        let email = json[constants::fields::EMAIL].as_str().unwrap_or("");

        let avatar = json[constants::fields::AVATAR_URL]
            .as_str()
            .unwrap_or(constants::fields::DEFAULT_AVATAR);

        let user = GitUser::new(
            String::from(user),
            String::from(email),
            String::from(avatar),
        );

        Ok(user)
    }

    fn get_body_as_json(body: Option<&String>, url: &str) -> Result<serde_json::Value, ApiError> {
        let url = Some(String::from(url));

        let Some(body) = body else {
            return Err(ApiError::no_body(url));
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
            return Err(ApiError::invalid_json(url));
        };

        Ok(json)
    }
}
