pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_question = text.trim_end().ends_with('?');
    let mut  only_letters = text.chars().filter(|c| c.is_alphabetic());
    let is_all_uppercase = only_letters.clone().count() > 0 && only_letters.all(|c| c.is_uppercase());

    if is_question {
        if is_all_uppercase {
            "Quiet, I am thinking!"
        } else {
            "Sure."
        }
    } else {
        if is_all_uppercase {
            "There is no need to yell, calm down!"
        } else {
            "Interesting"
        }
    }
}
