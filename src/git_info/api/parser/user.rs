use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::git_info::api::constants::fields;

use super::{constants, ApiError, GitUser, GitUserInfo, GitUserRepos, Parser};

impl Parser {
    // pub fn user(body: Option<&String>, url: &str) -> Result<GitUser, ApiError> {
    //     let json = Parser::get_body_as_json(body, url)?;

    //     Parser::user_from_value(&json, url)
    // }

    pub fn user_from_value(value: &serde_json::Value, url: &str) -> Result<GitUser, ApiError> {
        let url = Some(url.to_string());

        let Some(id) = value.get(constants::fields::ID) else {
            return Err(ApiError::field_not_found(constants::fields::ID, url));
        };

        let Some(id) = id.as_number() else {
            return Err(ApiError::field_invalid(constants::fields::ID, url));
        };

        let Some(id) = id.as_u64() else {
            return Err(ApiError::field_invalid(constants::fields::ID, url));
        };

        let Some(user) = value.get(constants::fields::LOGIN) else {
            return Err(ApiError::field_not_found(constants::fields::LOGIN, url));
        };

        let Some(user) = user.as_str() else {
            return Err(ApiError::field_invalid(constants::fields::LOGIN, url));
        };

        let email = value
            .get(constants::fields::EMAIL)
            .and_then(|email| email.as_str())
            .map(String::from);

        let avatar = value
            .get(constants::fields::AVATAR_URL)
            .and_then(|avatar| avatar.as_str())
            .unwrap_or(constants::fields::DEFAULT_AVATAR);

        let user = GitUser::new(
            id.to_string(),
            String::from(user),
            email,
            String::from(avatar),
        );

        Ok(user)
    }

    pub fn user_info(body: Option<&String>, url: &str) -> Result<GitUserInfo, ApiError> {
        let some_url = Some(url.to_string());

        let json = Parser::get_body_as_json(body, url)?;

        let user = Parser::user_from_value(&json, url)?;

        let name = json
            .get(constants::fields::NAME)
            .and_then(|email| email.as_str())
            .map(String::from);

        let public_repos = json
            .get(constants::fields::PUB_REPOS)
            .and_then(|public_repos| public_repos.as_u64())
            .unwrap_or(0) as u32;

        let public_gists = json
            .get(constants::fields::PUB_GISTS)
            .and_then(|public_gists| public_gists.as_u64())
            .unwrap_or(0) as u32;

        let followers = json
            .get(fields::FOLLOWERS)
            .and_then(|followers| followers.as_u64())
            .unwrap_or(0) as u32;

        let following = json
            .get(fields::FOLLOWING)
            .and_then(|following| following.as_u64())
            .unwrap_or(0) as u32;

        let private_repos = json
            .get(fields::PRIV_REPOS)
            .and_then(|private_repos| private_repos.as_u64())
            .unwrap_or(0) as u32;

        let private_gists = json
            .get(fields::PRIV_GIST)
            .and_then(|private_gists| private_gists.as_u64())
            .unwrap_or(0) as u32;

        let owned_private_repos = json
            .get(fields::OWNED_PRIV_REPOS)
            .and_then(|owned_private_repos| owned_private_repos.as_u64())
            .unwrap_or(0) as u32;

        let collaborators = json
            .get(fields::COLLABORATORS)
            .and_then(|collaborators| collaborators.as_u64())
            .unwrap_or(0) as u32;

        let disk_usage = json
            .get(fields::DISK_USAGE)
            .and_then(|disk_usage| disk_usage.as_u64())
            .unwrap_or(0) as u32;

        let Some(created_at) = json.get(constants::fields::CREATED_AT) else {
            return Err(ApiError::field_not_found(
                constants::fields::CREATED_AT,
                some_url,
            ));
        };

        let Some(created_at) = created_at.as_str() else {
            return Err(ApiError::field_invalid(
                constants::fields::CREATED_AT,
                some_url,
            ));
        };

        let Ok(created_at) = created_at.parse::<DateTime<Utc>>() else {
            let message = Some(format!(
                "Field {} is invalid: {} [{}]",
                constants::fields::CREATED_AT,
                created_at,
                url
            ));
            return Err(ApiError::field_invalid(created_at, message));
        };

        let Some(update_at) = json.get(constants::fields::UPDATED_AT) else {
            return Err(ApiError::field_not_found(
                constants::fields::UPDATED_AT,
                some_url,
            ));
        };

        let Some(update_at) = update_at.as_str() else {
            return Err(ApiError::field_invalid(
                constants::fields::UPDATED_AT,
                some_url,
            ));
        };

        let Ok(update_at) = update_at.parse::<DateTime<Utc>>() else {
            let message = Some(format!(
                "Field {} is invalid: {} [{}]",
                constants::fields::UPDATED_AT,
                update_at,
                url
            ));
            return Err(ApiError::field_invalid(update_at, message));
        };

        let user = Rc::new(user);

        let user_repo = GitUserRepos::new(
            public_repos,
            public_gists,
            followers,
            following,
            private_repos,
            private_gists,
            owned_private_repos,
            collaborators,
        );

        let user_info = GitUserInfo::new(user, name, user_repo, disk_usage, created_at, update_at);

        Ok(user_info)
    }
}

#[cfg(test)]
use super::UserJsonMock;

#[cfg(test)]
mod tests {

    use super::{Parser, UserJsonMock};

    #[test]
    fn test_user() {
        let mock_json = UserJsonMock::user();

        let user = Parser::user_info(Some(&mock_json), "url").unwrap();

        println!("{:?}", user);

        assert_eq!(user.user.login, "Juanjofp");
        assert_eq!(user.user.email, Some(String::from("juanjo@juanjofp.com")));

        // Todo: Review GitUse for events and info to check if the values are correct
        // and has minimun common values

        // Todo: Add more asserts
    }
}
