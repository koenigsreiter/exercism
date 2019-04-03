pub fn reply(message: &str) -> &str {
    let trimmed_msg: &str = message.trim();

    const NO_CONTENT_MSG: &str = "Fine. Be that way!";

    if trimmed_msg.is_empty() {
        NO_CONTENT_MSG
    } else {
        const YELLING_QUESTION_MSG: &str = "Calm down, I know what I'm doing!";
        const YELLING_MSG: &str = "Whoa, chill out!";
        const QUESTION_MSG: &str = "Sure.";
        const WHATEVER_MSG: &str = "Whatever.";

        let yelling: bool = message
            .chars()
            .filter(|input_char| input_char.is_alphabetic())
            .all(char::is_uppercase);
        let question: bool = message.trim().ends_with('?');
        let no_content: bool = trimmed_msg
            .chars()
            .filter(|input_char| input_char.is_alphabetic())
            .collect::<String>()
            .is_empty();

        match (yelling, question, no_content) {
            (true, true, false) => YELLING_QUESTION_MSG,
            (true, false, false) => YELLING_MSG,
            (_, true, _) => QUESTION_MSG,
            (false, false, true) => NO_CONTENT_MSG,
            _ => WHATEVER_MSG,
        }
    }
}
