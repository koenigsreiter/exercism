fn is_opening(bracket: char) -> bool {
    let opening_brackets = "([{";
    opening_brackets
        .chars()
        .any(|opening_bracket| opening_bracket == bracket)
}

fn get_opening_tag(bracket: char) -> char {
    match bracket {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => panic!("Non-Expected character {} received!", bracket),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets = "(){}[]";
    string
        .chars()
        .filter(|input_char| brackets.chars().any(|bracket| &bracket == input_char))
        .fold(Vec::new(), |mut brackets, bracket| {
            if is_opening(bracket) {
                brackets.push(bracket)
            } else if !brackets.is_empty()
                && get_opening_tag(bracket) == brackets[brackets.len() - 1]
            {
                brackets.pop();
            } else {
                brackets.push(bracket);
            }
            brackets
        })
        .is_empty()

    // unimplemented!(
    //     "Check if the string \"{:?}\" contains balanced brackets",
    //     &cleaned_string
    // );
}
