
use super::{constants, ApiError, Parser, GitUser};


impl Parser {


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

}
