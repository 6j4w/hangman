use hangman::{ build_game_data, check_ended, outro, clear_screen, get_input, Status, ALPHA_LOWER };

fn main() {
    let mut guess: char;
    let mut err: &str = "";

    let mut game_data = build_game_data();

    println!("Welcome! Let's play hangman!");
    loop {
        let found_letters_old = game_data.found_letters;

        check_ended(&mut game_data);

        if game_data.ended {
            outro(&game_data);
            break;
        }

        clear_screen();

        println!("\n{}", err);
        err = "";

        println!("Lives: {}", game_data.lives);

        for (char, status) in &game_data.word {
            // e.g. "_ e _ l"
            print!("{} ", if *status == Status::FOUND { char } else { &'_' });
        }

        println!("\nYou have guessed: {:?} ", game_data.wrong_letters);
        println!("\n\nGuess: ");

        guess = get_input();

        if !ALPHA_LOWER.contains(&guess) {
            eprintln!("Not a valid character.");
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
    }
}
