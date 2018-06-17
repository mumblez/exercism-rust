use std::collections::HashSet;
use std::iter::FromIterator;

pub fn check(candidate: &str) -> bool {
    let mut word = candidate.to_string();
    word = word.trim().replace(" ", "").replace("-", "").to_lowercase();
    let letters = HashSet::<char>::from_iter(word.chars());
    letters.len() == word.len()
}
