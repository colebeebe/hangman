use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;
use rand::Rng;

/// Reads the words from file. The words file must be at the root, called words.txt
/// 
/// # Returns
/// 
/// The words in the file as a vector of strings, wrapped in a result for error checking
fn read_words() -> std::io::Result<Vec<String>> {
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);

    let words = reader.lines()
        .filter_map(Result::ok)
        .collect();

    Ok(words)
}

/// Allows the player to guess a character
/// 
/// # Returns
/// 
/// The character that has been guessed
fn let_player_guess() -> char {
    let mut guess = String::new();
    
    loop {
        // We need to clear the guess, otherwise it will keep appending to `guess`
        guess.clear();

        // If we don't flush stdout, the prompt character won't appear in the right spot
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim the guess so it's just the character
        let guess = guess.trim();

        // If the guess is more than a single character, don't allow it
        if guess.len() != 1 {
            println!("Your guess must be a single letter.");
        } else {
            let c = guess.chars().next().unwrap();
            if c.is_alphabetic() {
                // If the player has put in a single letter, return it as lowercase
                return c.to_ascii_lowercase();
            } else {
                // We don't want to allow anything that's not a character, either
                println!("Your guess must be a letter.");
            }
        }
    }
}

/// Displays the man to be hanged, depending on the degree by which the player has guessed incorrectly
/// 
/// # Arguments
/// 
/// * `guesses` - A vector of chars that contains all guesses that the player has made
fn display_man(guesses: &Vec<char>) {
    println!("\n");
    // Print each guess for the player to reference
    for c in guesses {
        print!("{} ", c);
    }
    println!();
    let level = guesses.len();
    // Here, we enter a switch statement to print the graphic
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
        // Once we hit anything other than what we've already defined (which should always be 7),
        // the trapdoors open and the man meets his fate
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

/// Runs the game. This is where the main game loop is. The player is allowed to play until they guess
/// the correct word or the trapdoors open under the man's feet.
/// 
/// # Arguments
/// 
/// * `word` - The secret word.
fn play_game(word: &String) {
    // Create a few char vectors to hold the word as far as the player has decoded it and the wrong
    // guesses that the player has thus far made
    let mut decoded_word: Vec<char> = "_".repeat(word.chars().count()).chars().collect();
    let mut wrong_guesses: Vec<char> = Vec::new();

    // The main game loop. Don't exit until the player has decoded the entire word or has made too
    // many incorrect guesses
    while decoded_word.contains(&'_') && wrong_guesses.len() < 7 {
        display_man(&wrong_guesses);
        for c in &decoded_word {
            print!("{} ", c);
        }
        println!();
        let guess = let_player_guess();
        for (i, c) in word.chars().enumerate() {
            if c == guess {
                decoded_word[i] = guess;
            }
        }
        // If the decoded doesn't contain the player's guess by now, it must have been an incorrect
        // guess. If it's not already in `wrong_guesses`, then we should put it there (player is
        // not penalized for making the same wrong guess twice)
        if !decoded_word.contains(&guess) && !wrong_guesses.contains(&guess) {
            wrong_guesses.push(guess);
        }
    }

    // We've now exited the game loop. This means the player has either guessed the word or the
    // trapdoors have opened to the dismay and unfortune of the man. Display the man and the
    // player's decoded word one last time.
    display_man(&wrong_guesses);
    for c in &decoded_word {
        print!("{} ", c);
    }
    println!("\n");
    if decoded_word.contains(&'_') {
        // If the player wasn't able to guess the word, we should let them know what it was.
        println!("Sorry, you lose. The word was '{}'.\n", word);
    }
    else {
        // If the player won, let them know how many incorrect guesses they made
        println!("He walks free! You won with {} incorrect guesses!\n", wrong_guesses.len());
    }
}

/// The main function. THis is the entry point.
/// 
/// # Returns
/// 
/// The result from the file reading. Necessary if we want error catching.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read in the words
    let words = read_words()?;
    // Create a random number to index into the word vector
    let mut rng = rand::rng();
    let n: usize = rng.random_range(0..words.len());
    // Play the game
    play_game(&words[n]);
    Ok(())
}
