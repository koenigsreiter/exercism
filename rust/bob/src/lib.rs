pub fn reply(message: &str) -> &str {
    const NO_CONTENT_MSG: &str = "Fine. Be that way!";
    const YELLING_QUESTION_MSG: &str = "Calm down, I know what I'm doing!";
    const YELLING_MSG: &str = "Whoa, chill out!";
    const QUESTION_MSG: &str = "Sure.";
    const WHATEVER_MSG: &str = "Whatever.";

    let trimmed_msg: &str = message.trim();

    if trimmed_msg.is_empty() {
        NO_CONTENT_MSG
    } else {
        let yelling: bool = {
            let filtered_string: String = trimmed_msg
                .chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<String>();
            filtered_string.len() != 0
                && trimmed_msg
                    .chars()
                    .filter(char::is_ascii_alphabetic)
                    .all(char::is_uppercase)
        };
        let question: bool = trimmed_msg.ends_with('?');

        match (yelling, question) {
            (true, true) => YELLING_QUESTION_MSG,
            (true, false) => YELLING_MSG,
            (_, true) => QUESTION_MSG,
            _ => WHATEVER_MSG,
        }
    }
}
