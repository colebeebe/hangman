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


fn display_man(guesses: &Vec<char>) {
    println!("\n");
    for c in guesses {
        print!("{} ", c);
    }
    println!();
    let level = guesses.len();
    match level {
        0 => {
            println!("    |-----|");
            println!("          |");
            println!("          |");
            println!("          |");
            println!("          |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        1 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("          |");
            println!("          |");
            println!("          |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        2 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("    |     |");
            println!("    |     |");
            println!("          |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        3 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("    |     |");
            println!("    |     |");
            println!("   /      |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        4 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("    |     |");
            println!("    |     |");
            println!("   / \\    |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        5 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("  --|     |");
            println!("    |     |");
            println!("   / \\    |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        6 => {
            println!("    |-----|");
            println!("   ( )    |");
            println!("  --|--   |");
            println!("    |     |");
            println!("   / \\    |");
            println!(" _________|");
            println!("|         |");
            println!();
        }
        _ => {
            println!("    |-----|");
            println!("   (x)    |");
            println!("  --|--   |");
            println!("    |     |");
            println!("   / \\    |");
            println!(" _     ___|");
            println!("| \\   /   |");
            println!();
        }
    }

}


fn play_game(word: &String) {
    let mut decoded_word: Vec<char> = "_".repeat(word.chars().count()).chars().collect();
    let mut wrong_guesses: Vec<char> = Vec::new();

    while decoded_word.contains(&'_') && wrong_guesses.len() < 7 {
        display_man(&wrong_guesses);
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
        if !decoded_word.contains(&guess) && !wrong_guesses.contains(&guess) {
            wrong_guesses.push(guess);
        }
    }

    display_man(&wrong_guesses);
    for c in &decoded_word {
        print!("{} ", c);
    }
    println!("\n");
    if decoded_word.contains(&'_') {
        println!("Sorry, you lose. The word was '{}'.\n", word);
    }
    else {
        println!("He walks free! You won in {} guesses!\n", wrong_guesses.len());
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = read_words()?;
    let mut rng = rand::rng();
    let n: usize = rng.random_range(0..words.len());
    play_game(&words[n]);
    Ok(())
}
