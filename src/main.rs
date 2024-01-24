use hangman::{ Status, ALPHA_LOWER, GameData, clear_screen };
use std::io;

fn main() {
    let mut guess: String = String::new();
    let mut exit_mess: String = "You Lose!".to_string();
    //let mut game_data.lives: bool = false;
    let mut game_data = GameData {
        lives: 9,
        found_letters: 0,
        ended: false,
        word_readable: "heelinge".to_string(),
        word: vec![
            ('h', Status::NOTFOUND),
            ('e', Status::NOTFOUND),
            ('e', Status::NOTFOUND),
            ('l', Status::NOTFOUND),
            ('i', Status::NOTFOUND),
            ('n', Status::NOTFOUND),
            ('g', Status::NOTFOUND),
            ('e', Status::NOTFOUND)
        ],
    };

    println!("Welcome! Let's play hangman!");
    loop {
        let found_letters_old = game_data.found_letters;
        clear_screen();
        println!("Lives: {}", game_data.lives);
        for (char, status) in &game_data.word {
            print!("{} ", if *status == Status::FOUND { char } else { &'_' });
        }
        if game_data.ended {
            clear_screen();
            println!("\n{exit_mess}");
            println!("The word was: {0}", game_data.word_readable);
            println!("You have {} lives out of 9 left", game_data.lives);
            println!("Good game!");
            break;
        }

        println!("\n\nGuess: ");
        io::stdin().read_line(&mut guess).expect("Input went wrong");
        let first_char = guess.trim().to_lowercase().chars().next().unwrap();

        if !ALPHA_LOWER.contains(&first_char) {
            eprintln!("Not a valid character.");
        }
        for (c, status) in &mut game_data.word {
            if *c == first_char {
                *status = Status::FOUND;
                game_data.found_letters += 1;
            }
        }

        game_data.lives -= if found_letters_old == game_data.found_letters { 1 } else { 0 };
        game_data.ended = if game_data.lives == 0 { true } else { false };

        if game_data.word.iter().all(|(_, status)| *status == Status::FOUND) {
            exit_mess = "You win!".to_string();
            game_data.ended = true;
        }

        guess = "".to_string();
    }
}
