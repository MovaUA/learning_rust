mod chapter_08;

fn main() {
    // Given a list of integers,
    // use a vector and return
    // the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let empty: Vec<i32> = vec![];

    print_median_and_mode(&empty);
    print_median_and_mode(&vec![1]);
    print_median_and_mode(&vec![5, 3, 4]);
    print_median_and_mode(&vec![1, 2, 3, 2]);
    print_median_and_mode(&vec![1, 2, 1]);


    // Convert strings to pig latin.
    // The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!

    let original_text = "Hello, amazing world!";
    let pig_latin_text = chapter_08::convert_to_pig_latin(original_text);
    println!(
        "origin text:\n{}\npig latin:\n{}",
        original_text, pig_latin_text
    );
}


fn print_median_and_mode(v: &[i32]) {
    println!("{:?}", v);

    println!("median: {:?}", chapter_08::get_median(&v));
    println!("mode: {:?}", chapter_08::get_mode(&v));

    println!()
}

