pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let mut shouting = true;
    let mut nothing = true;
    let mut noletters = true;
    let mut question: bool = false;

    if message.ends_with("?") {
        question = true;
    }

    for c in message.chars() {
        if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
            nothing = false; 
            noletters = false; 
        }
        if c >= 'a' && c <= 'z' {
            shouting = false;
        }
        if c >= '0' && c <= '9' {
            nothing = false;
        }
    }

    if question && noletters { return "Sure." };
    if shouting && question { return "Calm down, I know what I'm doing!" };
    if question { return "Sure." };
    if shouting && !noletters { return "Whoa, chill out!" };
    if nothing { return "Fine. Be that way!" };
    "Whatever."
}
