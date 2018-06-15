pub fn verse(n: i32) -> String {

    let ofotw = "of beer on the wall";

    let num1: String = if n == 0 { "No more".to_string() } else { n.to_string() };
    let num2: String = if n-1 == 0 { "no more".to_string() } else { (n-1).to_string() };
    
    let bottle_first = if n == 1 { "bottle" } else { "bottles" };
    let bottle_second = if n-1 == 1 { "bottle" } else { "bottles" };
    let take = if n == 1 { "it" } else { "one" };

    if n != 0 {
        format!("{num1} {bottle_first} {ofotw}, {num1} {bottle_first} of beer.\nTake {take} down and pass it around, {num2} {bottle_second} {ofotw}.\n", 
                num1=num1,
                num2=num2,
                ofotw=ofotw,
                bottle_first=bottle_first,
                bottle_second=bottle_second,
                take=take,
                )
    } else {
        format!("{num1} {bottle_first} {ofotw}, no more {bottle_first} of beer.\nGo to the store and buy some more, 99 {bottle_second} {ofotw}.\n", 
                num1=num1,
                ofotw=ofotw,
                bottle_first=bottle_first,
                bottle_second=bottle_second,
                )
    }

}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();

    for v in (end..=start).rev() {
        song.push_str(verse(v).as_str());
        if v != end {
            song.push_str("\n");
        }
    }
    song
}
