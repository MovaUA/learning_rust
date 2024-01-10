mod median;
mod pig_latin;

fn main() {
    // Given a list of integers,
    // use a vector and return
    // the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let empty: Vec<i32> = vec![];

    do_the_job(&empty);
    do_the_job(&vec![1]);
    do_the_job(&vec![5, 3, 4]);
    do_the_job(&vec![1, 2, 3, 2]);
    do_the_job(&vec![1, 2, 1]);


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


fn do_the_job(v: &[i32]) {
    println!("{:?}", v);

    println!("median: {:?}", median::median(&v));
    println!("mode: {:?}", median::mode(&v));

    println!()
}

