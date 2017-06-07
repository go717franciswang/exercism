#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}

pub fn sublist<T>(l1: &[T], l2: &[T]) -> Comparison where T: PartialEq {
    if l1.len() == l2.len() {
        if equal(l1, l2) {
            return Comparison::Equal
        } else {
            return Comparison::Unequal
        }
    }

    if l1.len() < l2.len() {
        if l1.is_empty() 
            || l2.windows(l1.len()).any(|l2_sub| equal(l1, l2_sub)) {
            return Comparison::Sublist
        }
    } else {
        return match sublist(l2, l1) {
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            other => other
        }
    }

    Comparison::Unequal
}

fn equal<T>(l1: &[T], l2: &[T]) -> bool where T: PartialEq {
    l1.iter().zip(l2.iter()).all(|(a,b)| a == b)
}
