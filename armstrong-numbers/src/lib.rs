pub fn is_armstrong_number(num: u32) -> bool {
    let mut temp_num: u32 = num;
    let mut units: Vec<u32> = Vec::new();

    while temp_num != 0 {
        units.push(temp_num % 10);
        temp_num /= 10;
    }

    let power = units.len() as u32;

    units.iter().fold(0, |total, x| total + x.pow(power)) == num
}
