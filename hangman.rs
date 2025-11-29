/*
Author: Brandon Ayton
Date: 11-20-2025
Description: A simple Hangman Game in Rust where players guess
programming language names. Features inclue ASCII art, hint system, and a
word bank with descriptions. Demonstrates Rust concepts including structs,
HashMaps, slicing, and error handling.
*/

use std::io;
use std::collections::HashMap;

// Struct for game state
struct HangmanGame {
    secret_word: String,
    display_word: Vec<char>,
    guessed_letters: Vec<char>,
    wrong_guesses: i32,
    max_wrong: i32,
    word_bank: HashMap<&'static str, &'static str>, // Hashmap
}

impl HangmanGame {
    // Constructor
    fn new() -> Self {
        let word_bank = HashMap::from([
            ("RUST", "A systems programming language"),
            ("JAVA", "Write once, run anywhere"),
            ("SWIFT", "Apple's programming language"),
            ("PYTHON", "Known for its simplicity"),
            ("GOLANG", "Created by Google")
        ]);
        
        // Get a random word from the word bank
        let words: Vec<&str> = word_bank.keys().copied().collect();
        let secret_word = words[2].to_string(); // Using SWIFT
        
        HangmanGame {
            display_word: vec!['_'; secret_word.len()],
            guessed_letters: Vec::new(),
            wrong_guesses: 0,
            max_wrong: 6,
            word_bank,
            secret_word,
        }
    }
    
    // Method to display current game state
    fn display_game(&self) {
        println!("\n{}", "=".repeat(40));
        println!("HANGMAN GAME - GUESS THE PROGRAMMING LANGUAGE");
        println!("{}", "=".repeat(40));
        
        // Display the word with spaces between letters
        print!("Word: ");
        for letter in &self.display_word {
            print!("{} ", letter);
        }
        println!();
        
        // Display guessed letters
        if !self.guessed_letters.is_empty() {
            print!("Guessed letters: ");
            for letter in &self.guessed_letters {
                print!("{} ", letter);
            }
            println!();
        }
        
        // Display hangman status
        println!("Wrong guesses: {}/{}", self.wrong_guesses, self.max_wrong);
        self.display_hangman();
        
        // Give hint after 3 wrong guesses
        if self.wrong_guesses >= 3 {
            if let Some(hint) = self.word_bank.get(self.secret_word.as_str()) {
                println!("Hint: {}", hint);
            }
        }
    }
    
    // Method to display hangman ASCII art
    fn display_hangman(&self) {
        let stages = [
            "
            
            
            
            
            ",
            "
            
            
            
            
            ========
            ",
            "
            |
            |
            |
            |
            |
            ========
            ",
            "
            ______
            |
            |
            |
            |
            |
            ========
            ",
            "
            ______
            |    |
            |    O
            |
            |
            |
            ========
            ",
            "
            ______
            |    |
            |    O
            |   /|\\
            |
            |
            ========
            ",
            "
            ______
            |    |
            |    O
            |   /|\\
            |   / \\
            |
            ========
            "
        ];
        
        let stage_index = std::cmp::min(self.wrong_guesses as usize, stages.len() - 1);
        println!("{}", stages[stage_index]);
    }
    
    // Method to process a guess
    fn process_guess(&mut self, guess: char) -> bool {
        self.guessed_letters.push(guess);
        
        let mut correct_guess = false;
        
        // Check if guess is in secret word
        for (i, letter) in self.secret_word.chars().enumerate() {
            if letter == guess {
                self.display_word[i] = guess;
                correct_guess = true;
            }
        }
        
        if !correct_guess {
            self.wrong_guesses += 1;
        }
        
        correct_guess
    }
    
    // Method to check if game is won
    fn is_won(&self) -> bool {
        !self.display_word.contains(&'_')
    }
    
    // Method to check if game is lost
    fn is_lost(&self) -> bool {
        self.wrong_guesses >= self.max_wrong
    }
    
    // Method to reveal the word using slicing
    fn reveal_partial_word(&self) -> String {
        if self.secret_word.len() > 2 {
            // Use string slicing to show first and last character
            let first_char = &self.secret_word[0..1];
            let last_char = &self.secret_word[self.secret_word.len()-1..];
            format!("Starts with '{}', ends with '{}'", first_char, last_char)
        } else {
            "Word is too short for hints".to_string()
        }
    }
}

// Function to get valid letter input from user
fn get_player_input() -> char {
    loop {
        println!("\nEnter a single letter (A-Z):");
        
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cleaned_input = input.trim().to_uppercase();
                
                // Validate input
                if cleaned_input.len() == 1 {
                    let character = cleaned_input.chars().next().unwrap();
                    if character.is_alphabetic() {
                        return character;
                    }
                }
                
                println!("Please enter exactly one letter!");
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}

// Function to display game instructions
fn show_instructions() {
    println!("\nHOW TO PLAY:");
    println!("- Guess the programming language one letter at a time");
    println!("- You can make up to 6 wrong guesses");
    println!("- After 3 wrong guesses, you'll get a hint");
    println!("- Win by guessing all letters before running out of guesses");
    println!("{}", "=".repeat(40));
}

// Function to display game result
fn show_result(game: &HangmanGame, won: bool) {
    println!("\n{}", "=".repeat(40));
    if won {
        println!("YOU WIN!");
        println!("You guessed: {}", game.secret_word);
        println!("With {} wrong guesses remaining!", game.max_wrong - game.wrong_guesses);
    } else {
        println!("GAME OVER!");
        println!("The word was: {}", game.secret_word);
        if let Some(description) = game.word_bank.get(game.secret_word.as_str()) {
            println!("About {}: {}", game.secret_word, description);
        }
    }
    println!("{}", "=".repeat(40));
}

// Main game function
fn main() {
    println!("Welcome to Programming Language Hangman!");
    show_instructions();
    
    // Create new game instance
    let mut game = HangmanGame::new();
    
    // Main game loop
    loop {
        // Display current game state
        game.display_game();
        
        // Check game status
        if game.is_won() {
            show_result(&game, true);
            break;
        }
        
        if game.is_lost() {
            show_result(&game, false);
            break;
        }
        
        // Show additional hint using slicing
        if game.wrong_guesses == 2 {
            println!("{}", game.reveal_partial_word());
        }
        
        // Get player input
        let guess = get_player_input();
        
        // Check if letter was already guessed
        if game.guessed_letters.contains(&guess) {
            println!("You already guessed '{}'! Try a different letter.", guess);
            continue;
        }
        
        // Process the guess
        let correct = game.process_guess(guess);
        
        if correct {
            println!("Good guess! '{}' is in the word.", guess);
        } else {
            println!("Sorry, '{}' is not in the word.", guess);
        }
    }
}
