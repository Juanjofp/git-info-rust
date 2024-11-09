use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitRepository {
    name: String,
    description: String,
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
