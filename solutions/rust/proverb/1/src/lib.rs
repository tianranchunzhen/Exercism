pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut output = list
        .windows(2)
        .map(|sub_list| {
            format!(
                "For want of a {} the {} was lost.",
                sub_list[0], sub_list[1]
            )
        })
        .collect::<Vec<String>>();
    output.push(format!("And all for the want of a {}.", list[0]));
    output.join("\n")
}
