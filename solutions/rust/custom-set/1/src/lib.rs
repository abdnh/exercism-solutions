use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + Hash> {
    elements: HashSet<T>,
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            elements: HashSet::from_iter(input.iter().cloned()),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.elements.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.is_subset(&other.elements)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.elements.is_disjoint(&other.elements)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .intersection(&other.elements)
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.difference(&other.elements).cloned().collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.union(&other.elements).cloned().collect(),
        }
    }
}
