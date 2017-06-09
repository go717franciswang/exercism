extern crate alphametics;
use std::collections::HashMap;

fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
    let answer = alphametics::solve(puzzle).unwrap();
    let solution: HashMap<char, u8> = solution.iter().cloned().collect();
    assert_eq!(answer, solution);
}

#[test] 
fn test_digit_iter() {
    let mut iter = alphametics::DigitIter { n: 123, w: 3 };
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_digits() {
    let digits = alphametics::Digits { n: 123, w: 3 };
    assert_eq!(vec![3,2,1], digits.into_iter().collect::<Vec<u8>>());
}

#[test]
fn test_digit_permutation_iterator() {
    let mut iter = alphametics::DigitPermutationIterator::new(3);
    assert_eq!(Some(alphametics::Digits { n: 12, w: 3 }), iter.next());
    assert_eq!(Some(alphametics::Digits { n: 13, w: 3 }), iter.next());
    assert_eq!(Some(alphametics::Digits { n: 14, w: 3 }), iter.next());
}

#[test]
fn test_with_three_letters() {
    assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
}

#[test]
fn test_must_have_unique_value_for_each_letter() {
    let answer = alphametics::solve("A == B");
    assert_eq!(answer, None);
}

#[test]
fn test_leading_zero_solution_is_invalid() {
    let answer = alphametics::solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
fn test_puzzle_with_four_letters() {
    assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
}

#[test]
fn test_puzzle_with_six_letters() {
    assert_alphametic_solution_eq("NO + NO + TOO == LATE",
                &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]);
}

#[test]
fn test_puzzle_with_seven_letters() {
    assert_alphametic_solution_eq("HE + SEES + THE == LIGHT",
                &[('E', 4), ('G', 2), ('H', 5), ('I', 0), ('L', 1), ('S', 9), ('T', 7)]);
}

#[test]
fn test_puzzle_with_eight_letters() {
    assert_alphametic_solution_eq("SEND + MORE == MONEY",
                &[('S', 9), ('E', 5), ('N', 6), ('D', 7), ('M', 1), ('O', 0), ('R', 8), ('Y', 2)]);
}

#[test]
fn test_puzzle_with_ten_letters() {
    assert_alphametic_solution_eq("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
                &[('A', 5), ('D', 3), ('E', 4), ('F', 7), ('G', 8), ('N', 0), ('O', 2), ('R', 1),
                  ('S', 6), ('T', 9)]);
}
