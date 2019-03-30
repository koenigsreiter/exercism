pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::default();
    }

    let mut lines = list
        .iter()
        .take(list.len() - 1)
        .enumerate()
        .map(|(index, item)| format!("For want of a {} the {} was lost.", item, list[index + 1]))
        .collect::<Vec<String>>();
    lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")
    //unimplemented!("build a proverb from this list of items: {:?}", list)
}
