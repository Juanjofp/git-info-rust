use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitEvent {
    pub kind: String,

    pub created_at: DateTime<Utc>,
}

impl GitEvent {
    pub fn new(kind: String, created_at: DateTime<Utc>) -> Self {
        Self { kind, created_at }
    }
}

#[derive(Debug)]
pub struct GitEvents {
    events: RefCell<Vec<Rc<GitEvent>>>,
}

impl GitEvents {
    pub fn new() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn add(&self, event: GitEvent) {
        self.events.borrow_mut().push(Rc::new(event));
    }

    pub fn get(&self, index: usize) -> Option<Rc<GitEvent>> {
        let events = self.events.borrow();

        let event = events.get(index)?;

        Some(Rc::clone(event))
    }

    pub fn size(&self) -> usize {
        self.events.borrow().len()
    }
}

impl Default for GitEvents {
    fn default() -> Self {
        Self::new()
    }
}

// Iterator for GitEvents
pub struct GitEventsIter<'a> {
    events: &'a RefCell<Vec<Rc<GitEvent>>>,
    index: Cell<usize>,
}

impl GitEvents {
    pub fn iter(&self) -> GitEventsIter {
        GitEventsIter {
            events: &self.events,
            index: Cell::new(0),
        }
    }
}

impl<'a> Iterator for GitEventsIter<'a> {
    type Item = Rc<GitEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        let events = self.events.borrow();

        let index = self.index.get();

        if index >= events.len() {
            return None;
        }

        self.index.set(index + 1);

        let event = events.get(index)?;

        Some(Rc::clone(event))
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;

    use super::{DateTime, GitEvent, GitEvents};

    #[test]
    fn test_add_and_get_events() {
        let kind = "PushEvent".to_string();

        let created_at: DateTime<Utc> = "2022-06-09T12:47:28Z".parse().unwrap();

        let event_expected = GitEvent::new(kind, created_at);

        let events = GitEvents::new();

        events.add(event_expected.clone());

        assert_eq!(events.size(), 1);

        let event = events.get(0).unwrap();

        assert_eq!(*event, event_expected);
    }

    #[test]
    fn test_iter_events() {
        let kind = "PushEvent".to_string();

        let created_at: DateTime<Utc> = "2022-06-09T12:47:28Z".parse().unwrap();

        let event_expected = GitEvent::new(kind, created_at);

        let events = GitEvents::new();

        events.add(event_expected.clone());

        let mut iter = events.iter();

        let event = iter.next().unwrap();

        assert_eq!(*event, event_expected);
    }
}
