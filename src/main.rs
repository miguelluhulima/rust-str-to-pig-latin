use std::io;

fn main() {
    println!("Insert a sentence:");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line!");

    println!("Converted string:");

    for word in text.split_whitespace() {
        pigify(word);
    }
}

fn pigify(word: &str) -> String {
    let mut output = String::new();
    for (i, c) in word.chars().enumerate() {
        if is_vowel(c) {
            output = match i {
                x if x == 0 => format!("{}-hay", &word),
                _ => format!("{}-{}ay", &word[i..], &word[0..i]),
            };
            break;
        }
    }
    output
}

fn is_vowel(character: char) -> bool {
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    match vowels.iter().find(|&&v| v == character) {
        Some(_) => true,
        _ => false,
    }
}