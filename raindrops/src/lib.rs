pub fn raindrops(n: usize) -> String {
    let mut sound = String::new();

    if n % 3 == 0 { sound.push_str("Pling") };
    if n % 5 == 0 { sound.push_str("Plang") };
    if n % 7 == 0 { sound.push_str("Plong") };
    
    if sound.len() > 0 {
        sound.to_string()
    } else {
        n.to_string()
    }
}
