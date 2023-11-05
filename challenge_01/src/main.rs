mod linked_list;

use std::fs;

fn main() {
    let msg = fs::read_to_string("message.txt")
        .expect("Should have been able to read the file");

    // Split the string by whitespace into an iterator of words
    let mut words: Vec<String> = msg
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();

    // Init list with the first word
    let mut head = linked_list::WordList{
        value: words.remove(0),
        count: 1,
        next: linked_list::Address::Nil
    };

    // Append other words
    for word in words {
        head.append(word);
    }

    head.print_list();
}
