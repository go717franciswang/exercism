use std::collections::BTreeSet;
use std::iter::FromIterator;

#[derive(PartialEq, Debug)]
pub struct CustomSet<T> where T: Ord + Clone {
    set: BTreeSet<T>
}

impl<T> CustomSet<T> where T: Ord + Clone {
    pub fn new(v: Vec<T>) -> Self {
        let set: BTreeSet<T> = BTreeSet::from_iter(v.iter().cloned());
        CustomSet { set: set }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn contains(&self, x: &T) -> bool {
        unimplemented!();
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        unimplemented!();
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        unimplemented!();
    }

    pub fn add(&mut self, x: T) {
        unimplemented!();
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        unimplemented!();
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        unimplemented!();
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        unimplemented!();
    }
}
