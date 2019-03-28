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
    let cleaned_string = string
        .chars()
        .filter(|input_char| brackets.chars().any(|bracket| &bracket == input_char))
        .collect::<String>();

    let mut bracket_vec: Vec<char> = vec![];
    for bracket in cleaned_string.chars() {
        if is_opening(bracket) {
            bracket_vec.push(bracket)
        } else if !bracket_vec.is_empty()
            && get_opening_tag(bracket) == bracket_vec[bracket_vec.len() - 1]
        {
            bracket_vec.pop();
        } else {
            return false;
        }
    }

    bracket_vec.is_empty()

    // unimplemented!(
    //     "Check if the string \"{:?}\" contains balanced brackets",
    //     &cleaned_string
    // );
}
