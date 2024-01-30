use std::{ fs, io };
use rand::Rng;

#[derive(PartialEq)]
pub enum Status {
    FOUND,
    NOTFOUND,
}

pub static ALPHA_LOWER: [char; 26] = [
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

pub fn get_input() -> char {
    let mut guess: String = "".to_string();

    print!("Guess: ");
    io::stdin().read_line(&mut guess).expect("Input went wrong");

    guess.trim().to_lowercase().chars().next().expect("Input went wrong")
}

pub fn build_game_data() -> GameData {
    let word = get_readable();
    GameData {
        lives: 9,
        found_letters: 0,
        wrong_letters: Vec::new(), // <- this is for letters that were  wrong
        guessed_letters: Vec::new(),

        ended: false,
        ended_msg: "You Lose.".to_string(),

        word_readable: word.clone(),
        word: readable_to_word(word),
    }
}

pub fn check_ended(game_data: &mut GameData) {
    if game_data.word.iter().all(|(_, status)| *status == Status::FOUND) {
        game_data.ended = true;
        game_data.ended_msg = "You Win!".to_string();
    } else if game_data.lives == 0 {
        game_data.ended = true;
        game_data.ended_msg = "You lose...".to_string();
    }
}

pub fn outro(game_data: &GameData) {
    clear_screen();
    println!("\n{0}", game_data.ended_msg);
    println!("The word was: {0}", game_data.word_readable);
    println!("You have {} lives out of 9 left", game_data.lives);
    println!("Good game!");
}

fn get_readable() -> String {
    let contents = fs::read_to_string("words.dic").expect("Wordlist not readable");
    let mut lines = contents.lines();
    let rand_num = rand::thread_rng().gen_range(0..lines.clone().count());

    lines.nth(rand_num).expect("Retreiving random word didn't word").to_string()
}

fn readable_to_word(word_readable: String) -> Vec<(char, Status)> {
    let mut output: Vec<(char, Status)> = vec![];

    for c in word_readable.chars() {
        output.push((c, Status::NOTFOUND));
    }

    output
}
