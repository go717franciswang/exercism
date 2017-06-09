use std::collections::BTreeSet;
use std::iter::FromIterator;

#[derive(PartialEq, Debug)]
pub struct CustomSet<T> where T: Ord + Clone {
    set: BTreeSet<T>
}

impl<T> CustomSet<T> where T: Ord + Clone {
    pub fn new(v: Vec<T>) -> Self {
        let set: BTreeSet<T> = BTreeSet::from_iter(v);
        CustomSet { set: set }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn contains(&self, x: &T) -> bool {
        self.set.contains(x)
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.set.is_disjoint(&other.set)
    }

    pub fn add(&mut self, x: T) {
        self.set.insert(x);
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        Self::new(self.set.intersection(&other.set).cloned().collect::<Vec<T>>())
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        Self::new(self.set.difference(&other.set).cloned().collect::<Vec<T>>())
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        Self::new(self.set.union(&other.set).cloned().collect::<Vec<T>>())
    }
}
