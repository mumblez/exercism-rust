pub fn private_key(p: u64) -> u64 {
    // cheating to get primes without rng or rand crate
    let primes: Vec<u64> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];

    let idx = match primes.iter().position(|&x| x == p) {
        None => 0,
        Some(x) => x - 1,
    };

    primes[idx]
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
