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

pub struct GameData {
    pub lives: usize,
    pub found_letters: usize,

    pub ended: bool,

    pub word_readable: String,
    pub word: Vec<(char, Status)>,
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
