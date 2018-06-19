pub fn encode(source: &str) -> String {
    let mut last_char = ':';
    let mut count: u64 = 0;
    let mut result = String::new();

    for ch in source.chars() {
        match ch {
            'a'...'z' | ' ' | 'A'...'Z' => {
                if last_char == ':' {
                    last_char = ch;
                    count = 1;
                    continue;
                }
                if ch != last_char {
                    if count > 1 {
                        result.push_str(&format!("{}", count));
                    }
                    result.push(last_char);
                    last_char = ch;
                    count = 1;
                } else {
                    count += 1;
                }
            }
            _ => continue,
        }
    }

    if count > 1 {
        result.push_str(&format!("{}", count));
    }
    result.push(last_char);

    result
}

pub fn decode(source: &str) -> String {
    let mut count_string = String::new();
    let mut result = String::new();

    for ch in source.chars() {
        match ch {
            '0'...'9' => count_string.push(ch),
            'A'...'Z' | 'a'...'z' | ' ' => {
                if count_string.len() == 0 {
                    result.push(ch);
                } else {
                    let count = count_string.parse::<u64>().unwrap();
                    let repeated_chars = std::iter::repeat(ch)
                        .take(count as usize)
                        .collect::<String>();
                    result.push_str(&repeated_chars);
                    count_string.clear();
                }
            }
            _ => continue,
        }
    }
    result
}
