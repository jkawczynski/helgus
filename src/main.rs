use console::{style, Term};

fn main() {
    let terminal = Term::stdout();
    terminal.clear_screen().unwrap();
    let words = vec!["this", "is", "a", "long", "test"];

    for word in words {
        println!("Type the word: {}:", style(word).cyan());
        prompt_for_word(&terminal, word)
    }
}

fn prompt_for_word(terminal: &Term, word: &str) {
    let mut chars = word.chars();
    let mut target_char = chars.next().unwrap();

    loop {
        let char = terminal.read_char().unwrap();
        if char == target_char {
            target_char = match chars.next() {
                Some(ch) => ch,
                None => {
                    terminal.clear_screen().unwrap();
                    break;
                }
            }
        }
    }
}
