use super::{constants, ApiError, GitUser, Parser};

impl Parser {
    pub fn user(body: Option<&String>, url: &str) -> Result<GitUser, ApiError> {
        let json = Parser::get_body_as_json(body, url)?;

        Parser::user_from_value(&json, url)
    }

    pub fn user_from_value(value: &serde_json::Value, url: &str) -> Result<GitUser, ApiError> {
        let url = Some(url.to_string());

        let Some(user) = value.get(constants::fields::LOGIN) else {
            return Err(ApiError::field_not_found(constants::fields::LOGIN, url));
        };

        let Some(user) = user.as_str() else {
            return Err(ApiError::field_invalid(constants::fields::LOGIN, url));
        };

        let email = value
            .get(constants::fields::EMAIL)
            .and_then(|email| email.as_str());

        let avatar = value
            .get(constants::fields::AVATAR_URL)
            .and_then(|avatar| avatar.as_str())
            .unwrap_or(constants::fields::DEFAULT_AVATAR);

        let user = GitUser::new(
            String::from(user),
            email.map(String::from).unwrap_or_default(),
            String::from(avatar),
        );

        Ok(user)
    }
}
