/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean_isbn = isbn.replace("-", "");

    // false if not 10 digits minus dashes
    if clean_isbn.len() != 10 {
        return false;
    }

    // extract last check character
    let last: u64 = match clean_isbn.chars().last().unwrap() {
        'x' | 'X' => 10,
        n @ '1'...'9' => n.to_digit(10).unwrap() as u64,
        _ => return false,
    };

    // false if we're not left with digits (minus dashes and last check character
    if let Err(_) = clean_isbn[..clean_isbn.len() - 1].parse::<u64>() {
        return false;
    }

    let mut counter: u64 = 1;
    // safe to parse each character as a number if we reach here
    let total: u64 = clean_isbn
        .chars()
        .rev()
        .skip(1)
        .filter(|c| c.is_digit(10))
        .map(|c| {
            counter += 1;
            c.to_digit(10).unwrap() as u64 * counter
        })
        .sum();

    (total + last) % 11 == 0
}
