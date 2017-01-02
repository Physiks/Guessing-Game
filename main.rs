use std::io;
/*
    Guess the word given a description. You are given 3 tries.
    Meant to be played with 2 players.
*/
fn main() {
    let mut guess_word = String::new();
    let mut word_hint = String::new();
    let mut tries = 3;

    println!("Enter a word:");
    io::stdin().read_line(&mut guess_word).expect("Couldn't read input.");

    println!("Enter a hint:");
    io::stdin().read_line(&mut word_hint).expect("Couldn't read input.");

    while tries > 0 {
        print!("{}[2J", 27 as char);
        println!("{} \n{}", word_to_hidden(&guess_word.trim_right()), word_hint);
        println!("You have {} tries left.", tries);

        let mut cur_guess = String::new();
        match io::stdin().read_line(&mut cur_guess) {
        Ok(_) => {
            if cur_guess == guess_word {
                println!("You won!!!");
                break;
            } else {
                println!("Try again");
                tries -= 1;
            }
        }
        Err(e) => println!("Couldn't read input: {}", e),
    }
    }
    if tries == 0 {
        println!("You lost :(\nThe word was {}", guess_word);
    }
}

fn word_to_hidden(word: &str) -> String {
    let mut string_buf = String::with_capacity(word.len());

    for c in word.chars() {
        if c != ' ' {
            string_buf.push('-');
        } else {
            string_buf.push(' ');
        }
    }
    string_buf
}
