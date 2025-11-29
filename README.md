# Overview

This project is a simple console-based hangman game written in the Rust language.

The purpose for writing this project was to begin learning Rust, as previous to this I had never even seen the syntax for Rust, nor had I any previous knowledge as to any of its mechanics. As part of this project, I have included the simple programs I wrote while learning the language, found in each of the directories. They can all be run in the same way that the main project is run (as detailed below) by navigating to their respective directory instead of the hangman directory, which houses the main project.

[Software Demo Video](https://youtu.be/mhxfW6VZmPA)

# Development Environment

This project was written using Visual Studio Code.

The project was written in Rust, which deals strongly with ownership. All variables are 'owned' by a specific scope, and that scope cannot be violated. If the variable is transferred to another scope, it no longer belongs to its original scope, and therefore cannot be used by that scope any longer.

The only library that was used outside of `std` was `rand`, for choosing a random index in the list of words.

## Running the project

To run the project, first install rustup.

### On macOS or Linux

In the terminal run the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### On Windows

On Windows, the installer must be downloaded from [the Rust website](https://www.rust-lang.org/tools/install).

### Once rustup is installed

With rustup installed, you will want to make sure that cargo is installed:

```
cargo --version
```

which should return something like this:

```
cargo X.XX.X
```

With cargo installed, navigate to the hangman folder in the terminal and run:

```
cargo run
```

The console application should then run, and you can play the game.

# Useful Websites

- [Learn Rust](https://doc.rust-lang.org/book/) (Book from the Rust website)

# Future Work

Originally, the plan was to make a far more in-depth game than hangman. I originally wanted it to play a little more like a decision-based adventure, almost like Oregon Trail. However, as I studied Rust and learned that it was much more complicated than I originally believed, that had to be cut. That being said, I would like to try and implement this someday. It would include:

- A fantasy theme
- Encounters with different species and creatures
- Visitable towns
- Species and occupations for the player to choose from at the beginning of the game
- Interspecies relationships (for good or for bad - elves and dwarves hate each other, for example) 
- Interactions with species or locations that drive the player's decisions ('My party has an ent. Is it really that good of an idea to take a shortcut through a volcano?')
