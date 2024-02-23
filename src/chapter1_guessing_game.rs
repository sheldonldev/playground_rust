use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
    // welcome words
    println!("Game: guessing numbers! (type 'quit' to quit)");

    // loop the game
    'game: loop {
        // generate random secret number
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        // loop the guess till correct or quit
        'guess: loop {
            // get user input
            let mut guess: String = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            // if 'quit' then quit
            if guess.trim() == "quit" {
                println!("Quit the game.");
                break 'game;
            }

            // parse to number or continue input
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // comparison arms
            match guess.cmp(&secret_number) {
                Ordering::Greater => println!("Too large."),
                Ordering::Less => println!("Too small."),
                Ordering::Equal => {
                    println!("You win! One more time. (type 'quit' to quit)");
                    break 'guess;
                }
            }
        }
    }
}
