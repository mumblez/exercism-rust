use std::collections::HashSet;

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     (1..limit).filter(|&n| factors.iter().any(|f| n % f == 0)).sum()
// }
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total = HashSet::<u32>::new();

    for factor in factors {
        let mut current_factor = *factor;
        while current_factor < limit {
            total.insert(current_factor);
            current_factor += *factor;
        }
    }

    total.iter().sum()
}
