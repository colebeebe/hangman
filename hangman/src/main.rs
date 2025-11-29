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
    let mut decoded_word: Vec<char> = "_".repeat(word.chars().count()).chars().collect();

    while decoded_word.contains(&'_') {
        for c in &decoded_word {
            print!("{} ", c);
        }
        println!();
        let guess = let_user_guess();
        for (i, c) in word.chars().enumerate() {
            if c == guess {
                decoded_word[i] = guess;
            }
        }
        println!("Your guess: {}", guess);
    }

    for c in &decoded_word {
        print!("{} ", c);
    }
    println!();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = read_words()?;
    let mut rng = rand::rng();
    let n: usize = rng.random_range(0..words.len());
    println!("Random word: {}", words[n]);
    play_game(&words[n]);
    Ok(())
}
