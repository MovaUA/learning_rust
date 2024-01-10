use std::collections::HashMap;

mod pig_latin;

fn main() {
    // Convert strings to pig latin.
    // The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!

    let original_text = "Hello, amazing world!";
    let pig_latin_text = pig_latin::convert_to_pig_latin(original_text);
    println!(
        "origin text:\n{}\npig latin:\n{}",
        original_text, pig_latin_text
    );
}
