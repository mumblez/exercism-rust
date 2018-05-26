pub fn twofer(name: &str)-> String {
    let x = match name {
        name if name.is_empty() => "you",
        name => name
    };
    format!("One for {}, one for me.", x)
}

