pub fn collatz(n: u64) -> Option<u64> {
    let mut iterations: u64 = 0;
    let mut num: u64 = n;

    while num != 1 && n != 0 {
        iterations += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num *= 3;
            num += 1;
        }
    }

    if n > 0 {
        Some(iterations)
    } else {
        None
    }
}
