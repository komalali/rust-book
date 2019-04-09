pub fn translate(word: &str) -> String {
    let s = String::from(word);
    let first_character = s.chars().next();
    let vowel_conversion = format!("{}-hay", s);


    let pig_latin_string = match first_character {
        Some('a') => vowel_conversion,
        Some('e') => vowel_conversion,
        Some('i') => vowel_conversion,
        Some('o') => vowel_conversion,
        Some('u') => vowel_conversion,
        None => {
            println!("Can't convert an empty string to pig latin");
            String::from("")
        },
        Some(character) => format!("{}-{}ay", s[1..s.len()].to_owned(), character),
    };

    pig_latin_string
}
