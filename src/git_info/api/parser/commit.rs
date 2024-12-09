use std::rc::Rc;

use chrono::{DateTime, Utc};

use super::{constants, ApiError, GitCommit, GitCommits, Parser};

impl Parser {
    pub fn commits(body: Option<&String>, url: &str) -> Result<GitCommits, ApiError> {
        let json_array = Parser::get_body_as_json_array(body, url)?;

        let commits = json_array
            .iter()
            .fold(GitCommits::new(), |commits, commit_json| {
                let Ok(commit) = Parser::commit_from_value(commit_json, url) else {
                    return commits;
                };

                commits.add(commit);

                commits
            });

        Ok(commits)
    }

    // pub fn commit(body: Option<&String>, url: &str) -> Result<GitCommit, ApiError> {
    //     let json = Parser::get_body_as_json(body, url)?;

    //     Parser::commit_from_value(&json, url)
    // }

    fn commit_from_value(value: &serde_json::Value, url: &str) -> Result<GitCommit, ApiError> {
        let Some(commit) = value.get(constants::fields::COMMIT) else {
            return Err(ApiError::field_not_found(
                constants::fields::COMMIT,
                Some(url.to_string()),
            ));
        };

        let Some(author) = commit.get(constants::fields::AUTHOR) else {
            return Err(ApiError::field_not_found(
                constants::fields::AUTHOR,
                Some(url.to_string()),
            ));
        };

        let Some(author_date) = author.get(constants::fields::DATE) else {
            return Err(ApiError::field_not_found(
                constants::fields::DATE,
                Some(url.to_string()),
            ));
        };

        let Some(author_date) = author_date.as_str() else {
            return Err(ApiError::field_invalid(
                constants::fields::DATE,
                Some(url.to_string()),
            ));
        };

        let author_date = author_date
            .parse::<DateTime<Utc>>()
            .map_err(|_| ApiError::field_invalid(constants::fields::DATE, Some(url.to_string())))?;

        let Some(committer) = commit.get(constants::fields::COMMITTER) else {
            return Err(ApiError::field_not_found(
                constants::fields::COMMITTER,
                Some(url.to_string()),
            ));
        };

        let Some(committer_date) = committer.get(constants::fields::DATE) else {
            return Err(ApiError::field_not_found(
                constants::fields::DATE,
                Some(url.to_string()),
            ));
        };

        let Some(committer_date) = committer_date.as_str() else {
            return Err(ApiError::field_invalid(
                constants::fields::DATE,
                Some(url.to_string()),
            ));
        };

        let committer_date = committer_date
            .parse::<DateTime<Utc>>()
            .map_err(|_| ApiError::field_invalid(constants::fields::DATE, Some(url.to_string())))?;

        let message = commit
            .get(constants::fields::MESSAGE)
            .and_then(|m| m.as_str())
            .unwrap_or("Empty message");

        let Some(author) = value.get(constants::fields::AUTHOR) else {
            return Err(ApiError::field_not_found(
                constants::fields::AUTHOR,
                Some(url.to_string()),
            ));
        };

        let author = Parser::user_from_value(author, url)?;

        let Some(committer) = value.get(constants::fields::COMMITTER) else {
            return Err(ApiError::field_not_found(
                constants::fields::COMMITTER,
                Some(url.to_string()),
            ));
        };

        let committer = Parser::user_from_value(committer, url)?;

        let author = Rc::new(author);

        let committer = if *author == committer {
            Rc::clone(&author)
        } else {
            Rc::new(committer)
        };

        Ok(GitCommit::new(
            author,
            committer,
            message.to_string(),
            author_date,
            committer_date,
        ))
    }
}

#[cfg(test)]
use super::{CommitJsonMock, GitUser};

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use chrono::{DateTime, Utc};

    use super::{CommitJsonMock, GitCommit, GitCommits, GitUser, Parser};

    #[test]
    fn test_parse_json_commits() {
        let expected_commits = generate_commits();

        let commit_str = CommitJsonMock::commits();

        let commits = Parser::commits(Some(&commit_str), "url").unwrap();

        assert_eq!(commits.size(), expected_commits.size());

        let commit = commits.get(0).unwrap();

        assert_eq!(commit, expected_commits.get(0).unwrap());
    }

    fn generate_commits() -> GitCommits {
        let expected_user = GitUser::new(
            String::from("1"),
            "octocat".to_string(),
            None,
            "https://github.com/images/error/octocat_happy.gif".to_string(),
        );

        let expected_user = Rc::new(expected_user);

        let expected_commit = GitCommit::new(
            Rc::clone(&expected_user),
            Rc::clone(&expected_user),
            "Fix all the bugs".to_string(),
            "2011-04-14T16:00:49Z".parse::<DateTime<Utc>>().unwrap(),
            "2011-04-14T16:00:49Z".parse::<DateTime<Utc>>().unwrap(),
        );

        let expected_commits = GitCommits::new();
        expected_commits.add(expected_commit);

        expected_commits
    }
}
