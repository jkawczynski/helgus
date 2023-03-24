use console::{style, Term};

fn main() {
    let terminal = Term::stdout();
    terminal.clear_screen().unwrap();

    let words = vec!["This", " ", "is", " ", "a", " ", "long", " ", "test"];

    for (index, word) in words.iter().enumerate() {
        let line = get_active_line(index, &words);
        println!("{}", line);
        prompt_for_word(&terminal, word, get_guessed_line(index, &words))
    }
}

fn prompt_for_word(terminal: &Term, word: &str, guessed_line: String) {
    let mut guessed_chars = Vec::new();
    let mut chars = word.chars();
    let mut target_char = chars.next().unwrap();
    let mut guessed_chars_str = guessed_chars.iter().cloned().collect::<String>();
    terminal
        .write_line(&format!("{} {}", guessed_line, guessed_chars_str))
        .unwrap();

    loop {
        let char = terminal.read_char().unwrap();
        if char == target_char {
            guessed_chars.push(char);
            guessed_chars_str = guessed_chars.iter().cloned().collect::<String>();

            terminal.clear_last_lines(1).unwrap();
            terminal
                .write_line(&format!(
                    "{}{}",
                    guessed_line,
                    style(guessed_chars_str).green()
                ))
                .unwrap();

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

fn convert_space(word: &str) -> &str {
    match word {
        " " => "_",
        _ => word,
    }
}

fn get_active_line(active_word_index: usize, words: &Vec<&str>) -> String {
    words
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, word)| {
            if active_word_index == index {
                format!("{}", style(convert_space(word)).cyan())
            } else {
                format!("{}", style(word).dim())
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn get_guessed_line(active_word_index: usize, words: &Vec<&str>) -> String {
    words[..active_word_index]
        .iter()
        .cloned()
        .map(|word| format!("{}", style(word).green()))
        .collect::<Vec<String>>()
        .join("")
}
