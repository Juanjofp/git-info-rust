use std::{cell::Cell, cell::RefCell, rc::Rc};

use chrono::{DateTime, Utc};

use super::GitUser;

#[derive(Clone, Debug, PartialEq)]
pub struct GitCommit {
    author: Rc<GitUser>,
    committer: Rc<GitUser>,
    message: String,
    created_at: DateTime<Utc>,
    committed_at: DateTime<Utc>,
}

impl GitCommit {
    pub fn new(
        author: Rc<GitUser>,
        committer: Rc<GitUser>,
        message: String,
        created_at: DateTime<Utc>,
        committed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            author,
            committer,
            message,
            created_at,
            committed_at,
        }
    }
}

pub struct GitCommits {
    commits: RefCell<Vec<Rc<GitCommit>>>,
}

impl GitCommits {
    pub fn new() -> Self {
        Self {
            commits: RefCell::new(Vec::new()),
        }
    }

    pub fn add(&self, commit: GitCommit) {
        self.commits.borrow_mut().push(Rc::new(commit));
    }

    pub fn get(&self, index: usize) -> Option<Rc<GitCommit>> {
        let commits = self.commits.borrow();

        let commit = commits.get(index)?;

        let commit = Rc::clone(commit);

        Some(commit)
    }

    fn iter(&self) -> GitCommitIter {
        GitCommitIter {
            commits: &self.commits,
            index: Cell::new(0),
        }
    }
}

struct GitCommitIter<'a> {
    commits: &'a RefCell<Vec<Rc<GitCommit>>>,
    index: Cell<usize>,
}

impl<'a> Iterator for GitCommitIter<'a> {
    type Item = Rc<GitCommit>;

    fn next(&mut self) -> Option<Self::Item> {
        let commits = self.commits.borrow();

        let index = self.index.get();

        let commit = commits.get(index)?;

        self.index.set(index + 1);

        Some(Rc::clone(commit))
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, Utc};
    use std::rc::Rc;

    use super::{GitCommit, GitCommits, GitUser};

    #[test]
    fn test_add_and_get_commits() {
        let expected_commit = create_commit(String::from("Commit one"));

        let commits = GitCommits::new();

        commits.add(expected_commit.clone());

        let commit = commits.get(0).unwrap();

        assert_eq!(*commit, expected_commit);
    }

    #[test]
    fn test_commit_partial_equal() {
        let commit_one = create_commit(String::from("Commit one"));
        let commit_one_bis = create_commit(String::from("Commit one"));

        let commit_two = create_commit(String::from("Commit two"));

        assert_eq!(commit_one, commit_one_bis);
        assert_ne!(commit_one, commit_two);
    }

    #[test]
    fn test_iter_commits() {
        let commit_one = create_commit(String::from("One"));
        let commit_two = create_commit(String::from("Two"));

        let commits = GitCommits::new();
        commits.add(commit_one.clone());
        commits.add(commit_two.clone());

        let mut commits_iterator = commits.iter();

        let c1 = commits_iterator.next().unwrap();

        assert_eq!(*c1, commit_one);

        let c2 = commits_iterator.next().unwrap();

        assert_eq!(*c2, commit_two);
    }

    fn create_commit(message: String) -> GitCommit {
        let user = GitUser::new(
            String::from("Juanjo"),
            String::from("juanjo@juanjofp.com"),
            String::from("http://avatar_url.com"),
        );

        let author = Rc::new(user);
        let committer = Rc::clone(&author);

        let created_at = "2011-04-14T16:00:49Z".parse::<DateTime<Utc>>().unwrap();
        let committed_at = "2011-04-14T18:05:46Z".parse::<DateTime<Utc>>().unwrap();

        GitCommit::new(author, committer, message, created_at, committed_at)
    }
}
