use std::fs;

fn main() {
    let msg = fs::read_to_string("message.txt")
        .expect("Should have been able to read the file");

    let mut count = 0;

    for letter in msg.chars() {
        match letter {
            '#' => {
                count = count + 1;
            },
            '@' => {
                count = count - 1;
            },
            '*' => {
                count = count * count;
            },
            '&' => {
                print!("{}", count)
            },
            _ => {
                println!("Invalid command")
            }
        }
    }
}
