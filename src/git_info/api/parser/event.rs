use chrono::{DateTime, Utc};

use super::{constants, ApiError, GitEvent, GitEvents, Parser};

impl Parser {
    pub fn events(body: Option<&String>, url: &str) -> Result<GitEvents, ApiError> {
        let json_array = Parser::get_body_as_json_array(body, url)?;

        let events = json_array
            .iter()
            .fold(GitEvents::new(), |events, event_str| {
                let Ok(event) = Parser::event_from_value(event_str, url) else {
                    return events;
                };

                events.add(event);

                events
            });

        Ok(events)
    }

    //   pub fn parse_git_event(
    //     body: Option<&String>,
    //     url: &str,
    // ) -> Result<GitEvent, ApiError> {
    //     let json = Parser::get_body_as_json(body, url)?;

    //     Parser::parse_git_event_from_value(&json, url)
    // }

    fn event_from_value(value: &serde_json::Value, url: &str) -> Result<GitEvent, ApiError> {
        let some_url = Some(url.to_string());

        let Some(kind) = value.get(constants::fields::TYPE) else {
            return Err(ApiError::field_not_found(constants::fields::TYPE, some_url));
        };

        let Some(kind) = kind.as_str() else {
            return Err(ApiError::field_not_found(constants::fields::TYPE, some_url));
        };

        let Some(created_at) = value.get(constants::fields::CREATED_AT) else {
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

        Ok(GitEvent::new(kind.to_string(), created_at))
    }
}
