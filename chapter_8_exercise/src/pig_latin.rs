/*
Convert strings to pig latin. 
The first consonant of each word is moved to the end of the word and ay is added, 
so first becomes irst-fay. Words that start with a vowel have hay added to the end instead 
(apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */

 pub fn string_to_pig_latin(text: &str) -> String {
    let mut new_string  = String::new();
    for word in text.split_whitespace() {
        let first_letter = word.chars().next().unwrap();
        let last_char = word.chars().last().unwrap();
        let ay_letter: char;
        if is_vowel(first_letter) {
            ay_letter = 'h';
        } else {
            ay_letter = first_letter.to_ascii_lowercase();
        }
        let suffix = format!("-{}ay", ay_letter);
        new_string.push_str(&word[1..]);
        
        let punctuation = is_punctuation(last_char);
        if punctuation.is_some() {
            new_string.pop();
        }

        new_string.push_str(&suffix);
        if punctuation.is_some() {
            new_string.push(punctuation.unwrap());
        }

        new_string.push(' ');
    }

    new_string.pop();

    return new_string.to_string()
 }

 fn is_vowel(letter: char) -> bool {
    let c = letter.to_ascii_lowercase();
    if (c == 'a') || (c == 'e') || (c == 'i') || (c == 'o') || (c == 'u') {
        return true;
    }
    return false;
 }

  fn is_punctuation(c: char) -> Option<char> {
    if (c == '.') || (c == ',') || (c == ':') || (c == ';') || (c == '?') || (c == '!') {
        return Some(c);
    }
    return None;
 }

 // TODO: treat punctuation better; treat upper case words