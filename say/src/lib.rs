use std::collections::HashMap;

pub fn encode(n: u64) -> String {
    if n < 1 {
        return "zero".to_string();
    }
    let mut num = n;
    let mut response: Vec<String> = Vec::new();
    let mut numsections: Vec<u64> = Vec::new();
    let mut big = HashMap::new();
    big.insert(1, "thousand");
    big.insert(2, "million");
    big.insert(3, "billion");
    big.insert(4, "trillion");
    big.insert(5, "quadrillion");
    big.insert(6, "quintillion");

    loop {
        if num <= 0 {
            break;
        }
        numsections.push(num % 1000);
        num /= 1000
    }

    println!("numsections: {:#?}", numsections);

    for idx in (0..numsections.len()).rev() {
        let res = section_process(numsections[idx]);
        if res.len() > 0 {
            response.push(res);
        }
        if idx > 0 && numsections[idx] > 0 {
            response.push(big[&idx].into());
        }
    }

    response.join(" ")
}

fn section_process(n: u64) -> String {
    if n == 0 {
        return "".to_string();
    }
    let unit: u64 = n % 10;
    let ten: u64 = (n / 10) % 10;
    let hundred: u64 = (n / 100) % 100;

    let mut numreal = HashMap::new();
    numreal.insert(1, "one");
    numreal.insert(2, "two");
    numreal.insert(3, "three");
    numreal.insert(4, "four");
    numreal.insert(5, "five");
    numreal.insert(6, "six");
    numreal.insert(7, "seven");
    numreal.insert(8, "eight");
    numreal.insert(9, "nine");

    let mut teen = HashMap::new();
    teen.insert(10, "ten");
    teen.insert(11, "eleven");
    teen.insert(12, "twelve");
    teen.insert(13, "thirteen");
    teen.insert(14, "fourteen");
    teen.insert(15, "fifteen");
    teen.insert(16, "sixteen");
    teen.insert(17, "seventeen");
    teen.insert(18, "eighteen");
    teen.insert(19, "nineteen");

    let mut tens = HashMap::new();
    tens.insert(2, "twenty");
    tens.insert(3, "thirty");
    tens.insert(4, "forty");
    tens.insert(5, "fifty");
    tens.insert(6, "sixty");
    tens.insert(7, "seventy");
    tens.insert(8, "eighty");
    tens.insert(9, "ninety");

    let mut result: Vec<String> = Vec::new();

    if hundred > 0 {
        result.push(numreal[&hundred].into());
        result.push("hundred".into());
    }
    if unit == 0 && ten == 0 {
        return result.join(" ");
    }
    if ten == 1 {
        let tenx = ten * 10 + unit;
        result.push(teen[&tenx].into());
        return result.join(" ");
    }
    if ten > 1 {
        result.push(tens[&ten].into());
    }
    if unit == 0 {
        return result.join(" ");
    }
    if ten > 1 && unit > 0 {
        result.push("-".into());
    }
    result.push(numreal[&unit].into());

    result.join(" ").replace(" - ", "-").into()
}
