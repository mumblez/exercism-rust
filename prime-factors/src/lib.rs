pub fn factors(n: u64) -> Vec<u64> {
    let mut list: Vec<u64> = Vec::new();
    let mut current_num = n;
    let mut current_divisor = 2;

    loop {
        if current_num % current_divisor == 0 {
            current_num /= current_divisor;
            list.push(current_divisor);
        } else {
            if current_divisor == 2 {
                current_divisor += 1;
            } else {
                current_divisor += 2;
            }
        }
        if current_divisor > (n / 2) {
            break;
        }
    }

    list
}
