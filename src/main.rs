use std::io;

static ALPHA_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
fn main() {
    println!("Welcome! Let's play hangman!");
    let word = "hello".to_string();
    let mut guess: String = String::new();

    loop {
        guess.clear();
        println!("_ _ _ _");

        println!("\nGuess: ");
        io::stdin().read_line(&mut guess).expect("Input went wrong");
        guess = guess.trim().to_lowercase();

        if !ALPHA_LOWER.contains(&guess.chars().next().unwrap()) {
            eprintln!("Not a valid character. ");
        }

        if word.contains(guess.chars().next().unwrap()) {
            println!("[+]");
            println!("_ _ {guess} _");
        } else {
            println!("[-]");
        }
    }
}
