use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;
use rand::Rng;


fn read_words() -> std::io::Result<Vec<String>> {
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);

    let words = reader.lines()
        .filter_map(Result::ok)
        .collect();

    Ok(words)
}


fn let_user_guess() -> char {
    let mut guess = String::new();
    
    loop {
        guess.clear();

        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 1 {
            println!("Your guess must be a single letter.");
        } else {
            let c = guess.chars().next().unwrap();
            if c.is_alphabetic() {
                return c.to_ascii_lowercase();
            } else {
                println!("Your guess must be a letter.");
            }
        }
    }
}

fn play_game(word: &String) {
    let mut decoded_word = "_ ".repeat(word.chars().count());
    println!("Word: {}", decoded_word);

    let guess = let_user_guess();
    println!("Your guess: {}", guess);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = read_words()?;
    let mut rng = rand::rng();
    let n: usize = rng.random_range(0..words.len());
    println!("Random word: {}", words[n]);
    play_game(&words[n]);
    Ok(())
}
