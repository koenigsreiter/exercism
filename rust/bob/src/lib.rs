pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }

    let yelling = message
        .chars()
        .filter(|input_char| input_char.is_alphabetic())
        .all(char::is_uppercase);
    let question = message.trim().ends_with('?');
    let no_content = message
        .trim()
        .chars()
        .filter(|input_char| input_char.is_alphabetic())
        .collect::<String>()
        .is_empty();

    match (yelling, question, no_content) {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, false) => "Whoa, chill out!",
        (_, true, _) => "Sure.",
        (false, false, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
