#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2).filter(|x| num % x == 0).sum::<u64>() {
        _ if num == 0 => None,
        n if n < num => Some(Classification::Deficient),
        n if n > num => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}
