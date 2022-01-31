pub fn pig_latin(slice: &str) -> String {
    let mut sentence = String::new();
    for word in slice.split_whitespace() {
        if let Some(first) = word.chars().nth(0) {
            if ['a', 'e', 'i', 'o', 'u'].contains(&first) {
                let piggy = format!("{}-hay", word);
                sentence.push(' ');
                sentence.push_str(piggy.as_str());
            } else {
                let mut rest_of_word = word.chars();
                rest_of_word.next();
                let piggy = format!("{}-{}ay", rest_of_word.as_str(), first);
                sentence.push(' ');
                sentence.push_str(piggy.as_str());
            }
        }
    }
    sentence.trim().to_string()
}
