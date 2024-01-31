use std::{ fs, io, process };
use rand::Rng;

static ALPHA_LOWER: [char; 26] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
];

// public
#[derive(PartialEq)]
pub enum Status {
    FOUND,
    NOTFOUND,
}

// basic structure for the game
pub struct GameData {
    pub lives: usize,
    pub found_letters: usize,
    pub wrong_letters: Vec<char>,
    pub guessed_letters: Vec<char>,

    pub ended: bool,
    pub ended_msg: String,

    pub word_readable: String,
    pub word: Vec<(char, Status)>,
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn fatal_error(err: &str) {
    eprintln!("{err}");
    process::exit(1);
}

pub fn get_input() -> Result<char, &'static str> {
    let mut guess: String = "".to_string();

    print!("Guess: ");
    io::stdin().read_line(&mut guess).expect("Input went wrong");

    match guess.trim().to_lowercase().chars().next() {
        Some(t) => Ok(t),
        None => Err("Error occured while parsing guess."),
    }
}

pub fn validate_guess(guess: &char) -> bool {
    if ALPHA_LOWER.contains(guess) {
        return true;
    }
    false
}

pub fn build_game_data() -> Result<GameData, &'static str> {
    let word = match get_readable() {
        Ok(t) => t,
        Err(_) => {
            return Err("Error while retrieving word.");
        }
    };
    Ok(GameData {
        lives: 9,
        found_letters: 0,
        wrong_letters: Vec::new(), // <- this is for letters that were  wrong
        guessed_letters: Vec::new(),

        ended: false,
        ended_msg: "You Lose.".to_string(),

        word_readable: word.clone(),
        word: readable_to_word(word),
    })
}

pub fn check_ended(game_data: &mut GameData) {
    if game_data.word.iter().all(|(_, status)| *status == Status::FOUND) {
        game_data.ended = true;
        game_data.ended_msg = "You win!".to_string();
    } else if game_data.lives == 0 {
        game_data.ended = true;
        game_data.ended_msg = "You lose...".to_string();
    }
    if game_data.ended {
        outro(&game_data)
    }
}

// private

fn get_readable() -> Result<String, &'static str> {
    let contents = fs::read_to_string("words.dic").expect("Wordlist inacessible.");
    let mut lines = contents.lines();
    let rand_num = rand::thread_rng().gen_range(0..lines.clone().count());

    match lines.nth(rand_num) {
        Some(t) => Ok(t.to_string()),
        None => Err("Error retrieving word. Perhaps the wordlist is not accessible?"),
    }
}

fn readable_to_word(word_readable: String) -> Vec<(char, Status)> {
    let mut output: Vec<(char, Status)> = vec![];

    for c in word_readable.chars() {
        output.push((c, Status::NOTFOUND));
    }

    output
}

fn outro(game_data: &GameData) {
    clear_screen();
    println!("\n{0}", game_data.ended_msg);
    println!("The word was: {0}", game_data.word_readable);
    println!("You have {} lives out of 9 left", game_data.lives);
    println!("Good game!");
}
