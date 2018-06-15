pub fn build_proverb(list: Vec<&str>) -> String {
    let mut full_proverb = String::new();

    if !list.is_empty() {
        list.windows(2).for_each(|w| {
            full_proverb.push_str(&format!("For want of a {} the {} was lost.\n", w[0], w[1]));
        });

        full_proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    full_proverb

}
