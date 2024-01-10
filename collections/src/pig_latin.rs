use phf::phf_map;

const AVERAGE_BYTES_PER_WORD: usize = 4;
const AVERAGE_WORD_BYTES_INCREMENT: usize = 4;

pub fn convert_to_pig_latin(text: &str) -> String {
    let mut pig_latin = String::with_capacity(
        text.len() / AVERAGE_BYTES_PER_WORD
            * (AVERAGE_BYTES_PER_WORD + AVERAGE_WORD_BYTES_INCREMENT),
    );

    for word in get_words(text) {
        match word.into() {
            Token::Word(first_char) => {
                pig_latin.push_str(&convert_word_to_pig_latin(word, first_char)[..])
            }
            Token::NonWord() => pig_latin.push_str(word),
        }
    }

    pig_latin
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Word(Char),
    NonWord(),
}

#[derive(Copy, Clone, PartialEq)]
enum Char {
    Consonant(),
    Vowel(),
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match CHARS.get(&value) {
            Some(word_char) => Token::Word(*word_char),
            None => Token::NonWord(),
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        if let Some(first_char) = value.chars().next() {
            return Token::from(first_char);
        }

        Token::NonWord()
    }
}

const CHARS: phf::Map<char, Char> = phf_map! {
    'a'=> Char::Vowel(),
    'b'=> Char::Consonant(),
    'c'=> Char::Consonant(),
    'd'=> Char::Consonant(),
    'e'=> Char::Vowel(),
    'f'=> Char::Consonant(),
    'g'=> Char::Consonant(),
    'h'=> Char::Consonant(),
    'i'=> Char::Vowel(),
    'j'=> Char::Consonant(),
    'k'=> Char::Consonant(),
    'l'=> Char::Consonant(),
    'm'=> Char::Consonant(),
    'n'=> Char::Consonant(),
    'o'=> Char::Vowel(),
    'p'=> Char::Consonant(),
    'q'=> Char::Consonant(),
    'r'=> Char::Consonant(),
    's'=> Char::Consonant(),
    't'=> Char::Consonant(),
    'u'=> Char::Vowel(),
    'v'=> Char::Consonant(),
    'w'=> Char::Consonant(),
    'x'=> Char::Consonant(),
    'y'=> Char::Consonant(),
    'z'=> Char::Consonant(),
    'A'=> Char::Vowel(),
    'B'=> Char::Consonant(),
    'C'=> Char::Consonant(),
    'D'=> Char::Consonant(),
    'E'=> Char::Vowel(),
    'F'=> Char::Consonant(),
    'G'=> Char::Consonant(),
    'H'=> Char::Consonant(),
    'I'=> Char::Vowel(),
    'J'=> Char::Consonant(),
    'K'=> Char::Consonant(),
    'L'=> Char::Consonant(),
    'M'=> Char::Consonant(),
    'N'=> Char::Consonant(),
    'O'=> Char::Vowel(),
    'P'=> Char::Consonant(),
    'Q'=> Char::Consonant(),
    'R'=> Char::Consonant(),
    'S'=> Char::Consonant(),
    'T'=> Char::Consonant(),
    'U'=> Char::Vowel(),
    'V'=> Char::Consonant(),
    'W'=> Char::Consonant(),
    'X'=> Char::Consonant(),
    'Y'=> Char::Consonant(),
    'Z'=> Char::Consonant(),
};

#[derive(Copy, Clone)]
struct CharIndex {
    index: usize,
    char: char,
    token: Token,
}

fn get_words(text: &str) -> Vec<&str> {
    let mut words = Vec::with_capacity(text.len() / AVERAGE_BYTES_PER_WORD);

    let mut iter = get_word_boundaries(text).into_iter();

    let mut prev_index = 0;

    for curr_index in iter {
        let word = &text[prev_index..curr_index];
        words.push(word);
        prev_index = curr_index;
    }

    words.push(&text[prev_index..text.len()]);

    words
}

fn get_word_boundaries(text: &str) -> Vec<usize> {
    if text.len() == 0 {
        return vec![];
    }

    let average_word_chars_count = 4;
    let mut word_boundaries = Vec::with_capacity(text.len() / average_word_chars_count);

    let mut prev: Option<Token> = None;

    for (index, char) in text.char_indices() {
        let curr: Token = char.into();

        match (prev, curr) {
            (Some(Token::NonWord()), Token::Word(_)) => {
                // boundary of the non-word
                word_boundaries.push(index);
            }
            (Some(Token::Word(_)), Token::NonWord()) => {
                // boundary of the word
                word_boundaries.push(index);
            }
            (_, _) => {}
        }

        prev = Some(curr);
    }

    word_boundaries
}

fn convert_word_to_pig_latin(word: &str, first_char: Char) -> String {
    match first_char {
        Char::Consonant() => format!("{}-{}ay", &word[1..], &word[..1]),
        Char::Vowel() => format!("{}-hay", word),
    }
}

#[cfg(test)]
mod test {
    use crate::pig_latin;
    use crate::pig_latin::{convert_word_to_pig_latin, Char};

    #[test]
    fn convert_to_pig_latin() {
        // arrange
        let text = "Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!";

        // act
        let actual = pig_latin::convert_to_pig_latin(text);

        // assert
        let expected = "onvert-Cay trings-say o-tay ig-pay atin-lay. he-Tay irst-fay onsonant-cay of-hay each-hay ord-way is-hay oved-may o-tay he-tay end-hay of-hay he-tay ord-way and-hay “ay-hay” is-hay added-hay, o-say “irst-fay” ecomes-bay “irst-hay-ay-fay.” ords-Way hat-tay tart-say ith-way a-hay owel-vay ave-hay “ay-hay” added-hay o-tay he-tay end-hay instead-hay (“apple-hay” ecomes-bay “apple-hay-ay-hay”). eep-Kay in-hay ind-may he-tay etails-day about-hay UTF-hay-8 encoding-hay!";
        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_to_pig_latin_empty() {
        // arrange
        let text = "";

        // act
        let actual = pig_latin::convert_to_pig_latin(text);

        // assert
        let expected = "";
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_words() {
        // arrange
        let text = "hello, world!";

        // act
        let actual = pig_latin::get_words(text);

        // assert
        let expected = vec!["hello", ", ", "world", "!"];
        assert_eq!(actual, expected, "the input was:\n{}", text);
    }

    #[test]
    fn get_words_empty() {
        // arrange
        let text = "";

        // act
        let actual = pig_latin::get_words(text);

        // assert
        let expected = vec![""];
        assert_eq!(actual, expected, "the input was:\n{}", text);
    }

    #[test]
    fn get_word_boundaries() {
        // arrange

        // 0123456789
        // hi, there!
        // __2_4____9
        let text = "hi, there!";

        // act
        let actual = pig_latin::get_word_boundaries(text);

        // assert
        assert_eq!(actual, vec![2, 4, 9], "the text was:\n{}", text);
    }

    #[test]
    fn get_word_boundaries_empty() {
        // arrange
        let text = "";

        // act
        let actual = pig_latin::get_word_boundaries(text);

        // assert
        assert_eq!(actual, vec![], "the text was:\n{}", text);
    }

    #[test]
    fn convert_word_to_pig_latin_word_starts_with_consonant() {
        // arrange
        let word = "hello";
        let first_char = Char::Consonant();

        // act
        let actual = convert_word_to_pig_latin(&word, first_char);

        // Asser.
        assert_eq!(actual, "ello-hay")
    }

    #[test]
    fn convert_word_to_pig_latin_word_starts_with_vowel() {
        // arrange
        let word = "atrium";
        let first_char = Char::Vowel();

        // act
        let actual = convert_word_to_pig_latin(&word, first_char);

        // assert
        assert_eq!(actual, "atrium-hay")
    }
}
