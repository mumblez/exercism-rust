pub fn nth(n: u32) -> Option<u32> {
    if n < 1 { return None }
    if n == 1 { return Some(2) };

    let mut prime: bool;
    let mut pns_found = 1;
    let mut current_number = 3;

    loop {
        prime = false;
        
        let limit;
        if current_number > 5 {
            limit = current_number / 2;
        } else {
            limit = current_number;
        }
            
        for n in 2..limit {
            prime = true;
            if current_number % n == 0 {
                prime = false;
                break;
            }

        }

        if prime {
            pns_found += 1;
        }

        if pns_found == n {
            return Some(current_number);
        }
        current_number += 1;
    }
}
