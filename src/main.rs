use sadman::{
    build_game_data,
    check_ended,
    clear_screen,
    get_input,
    validate_guess,
    GameData,
    Status,
};

use std::process;

fn main() {
    let mut guess: char;
    let mut err: &str = "";

    let mut game_data: GameData = match build_game_data() {
        Ok(t) => t,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };

    println!("Welcome! Let's play hangman!");
    loop {
        let found_letters_old = game_data.found_letters;

        check_ended(&mut game_data);

        if game_data.ended {
            break;
        }

        clear_screen();

        println!("Sadman v1\n");
        if !err.is_empty() {
            println!("{}", err);
            err = "";
        }
        println!("Lives: {}", game_data.lives);

        for (char, status) in &game_data.word {
            // e.g. "_ e _ l"
            print!("{} ", if *status == Status::FOUND { char } else { &'_' });
        }

        println!("\nYou have tried: {:?} ", game_data.wrong_letters);
        println!("\n\nGuess: ");

        guess = match get_input() {
            Ok(t) => t,
            Err(e) => {
                err = e;
                continue;
            }
        };

        if !validate_guess(&guess) {
            err = "Guess not valid.";
            continue;
        }

        if game_data.guessed_letters.iter().any(|c| c == &guess) {
            err = "You already guessed this!";
            continue;
        }

        for (c, status) in &mut game_data.word {
            if *c == guess {
                *status = Status::FOUND;
                game_data.found_letters += 1;
            }
        }

        game_data.lives -= if found_letters_old == game_data.found_letters {
            game_data.wrong_letters.push(guess);
            1
        } else {
            0
        };

        game_data.guessed_letters.push(guess);
    }
}
