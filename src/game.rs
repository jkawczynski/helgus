use core::fmt;

use console::{style, Term};

pub struct TypingGame {
    terminal: Term,
}

impl Default for TypingGame {
    fn default() -> TypingGame {
        return TypingGame {
            terminal: Term::stdout(),
        };
    }
}

impl TypingGame {
    pub fn start_game(&mut self, sentence: &str) -> Result<(), std::io::Error> {
        self.terminal.clear_screen()?;

        let mut words = get_sentence_words(sentence);
        let mut cursor_x_position = 0;
        let mut cursor_y_position = 0;


        println!("{}", sentence);

        for word in words.iter_mut() {
            while !word.guessed {
                let (x_position, y_position) =
                    self.prompt_for_chars(word, cursor_x_position, cursor_y_position);
                word.set_guessed();
                cursor_x_position = x_position;
                cursor_y_position = y_position;
            }
        }
        Ok(())
    }

    fn prompt_for_chars(
        &self,
        game_word: &mut GameWord,
        cursor_x_position: usize,
        cursor_y_position: usize,
    ) -> (usize, usize) {
        let (_, terminal_cols) = &self.terminal.size_checked().unwrap();
        let mut chars = game_word.word.chars();
        let mut target_char = chars.next().unwrap();

        let mut x_pos = cursor_x_position;
        let mut y_pos = cursor_y_position;

        loop {
            self.terminal.move_cursor_to(x_pos, y_pos).unwrap();
            let char = self.terminal.read_char().unwrap();
            if char == target_char {
                println!("{}", style(char).green());

                x_pos += 1;
                if x_pos == *terminal_cols as usize {
                    x_pos = 0;
                    y_pos += 1;

                }
                target_char = match chars.next() {
                    Some(ch) => ch,
                    None => return (x_pos, y_pos),
                };
            } else {
                println!("{}", style(target_char).red());
                game_word.increment_error();
            }
        }
    }
}

fn get_sentence_words(sentence: &str) -> Vec<GameWord> {
    let words = sentence.split(" ").collect::<Vec<&str>>();
    let mut result: Vec<GameWord> = Vec::new();

    let stop_at_index = words.len() - 1;
    for (index, word) in words.iter().enumerate() {
        result.push(GameWord::new(word));
        if index != stop_at_index {
            result.push(GameWord::new(" "));
        }
    }
    result
}

struct GameWord<'a> {
    word: &'a str,
    error_count: i32,
    guessed: bool,
}

impl GameWord<'_> {
    fn new(word: &str) -> GameWord {
        GameWord {
            word,
            error_count: 0,
            guessed: false,
        }
    }

    fn increment_error(&mut self) {
        self.error_count += 1;
    }

    fn set_guessed(&mut self) {
        self.guessed = true;
    }

    fn should_count_to_score(&self) -> bool {
        self.word != " "
    }
}
