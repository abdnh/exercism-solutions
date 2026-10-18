pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".into();
    }
    let footer = format!("And all for the want of a {}.", list[0]);
    let mut lines = vec![];
    let mut previous = list[0];
    for i in 1..list.len() {
        lines.push(format!(
            "For want of a {} the {} was lost.",
            previous, list[i]
        ));
        previous = list[i];
    }
    if lines.is_empty() {
        footer
    } else {
        format!("{}\n{}", lines.join("\n"), footer)
    }
}
