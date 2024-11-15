use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitRepository {
    pub name: String,
    pub description: String,
}

impl GitRepository {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[derive(Debug)]
pub struct GitRepositories {
    repositories: RefCell<Vec<Rc<GitRepository>>>,
}

impl GitRepositories {
    pub fn new() -> Self {
        Self {
            repositories: RefCell::new(Vec::new()),
        }
    }

    pub fn add_repository(&self, repository: GitRepository) {
        self.repositories.borrow_mut().push(Rc::new(repository));
    }

    pub fn get_repository(&self, index: usize) -> Option<Rc<GitRepository>> {
        let repositories = self.repositories.borrow();

        let repository = repositories.get(index)?;

        Some(Rc::clone(repository))
    }

    pub fn size(&self) -> usize {
        self.repositories.borrow().len()
    }
}

impl Default for GitRepositories {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GitRepositoriesIter<'a> {
    repositories: &'a RefCell<Vec<Rc<GitRepository>>>,
    index: Cell<usize>,
}

impl<'a> Iterator for GitRepositoriesIter<'a> {
    type Item = Rc<GitRepository>;

    fn next(&mut self) -> Option<Self::Item> {
        let repositories = self.repositories.borrow();

        let index = self.index.get();

        if index >= repositories.len() {
            return None;
        }

        let repository = Rc::clone(&repositories[index]);

        self.index.set(index + 1);

        Some(repository)
    }
}

impl GitRepositories {
    pub fn iter(&self) -> GitRepositoriesIter {
        GitRepositoriesIter {
            repositories: &self.repositories,
            index: Cell::new(0),
        }
    }
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::{GitRepositories, GitRepository};

    #[test]
    fn test_add_and_get_repository() {
        let git_repositories = GitRepositories::new();

        let git_repository = GitRepository::new("repo1".to_string(), "description1".to_string());

        git_repositories.add_repository(git_repository);

        assert_eq!(git_repositories.size(), 1);

        let repo = git_repositories.get_repository(0).unwrap();

        assert_eq!(repo.name, "repo1".to_string());

        assert_eq!(repo.description, "description1".to_string());
    }

    #[test]
    fn test_iter_repository() {
        let repo1_name = "repo1".to_string();
        let repo1_description = "description1".to_string();

        let repo2_name = "repo2".to_string();
        let repo2_description = "description2".to_string();

        let git_repositories = GitRepositories::new();

        git_repositories.add_repository(GitRepository::new(
            repo1_name.clone(),
            repo1_description.clone(),
        ));

        git_repositories.add_repository(GitRepository::new(
            repo2_name.clone(),
            repo2_description.clone(),
        ));

        let content_as_vector = git_repositories.iter().collect_vec();

        assert_eq!(content_as_vector.len(), 2);
        assert_eq!(content_as_vector[0].name, repo1_name);
        assert_eq!(content_as_vector[0].description, repo1_description);
        assert_eq!(content_as_vector[1].name, repo2_name);
        assert_eq!(content_as_vector[1].description, repo2_description);
    }
}
