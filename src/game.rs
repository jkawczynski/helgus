use core::fmt;
use std::time::Instant;

use console::{style, Term};

pub struct GameResult {
    wpm: u64,
    time_in_seconds: u64,
    accuracy: f32,
    words_count: u64,
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WPM: {}\nAccuracy: {:.2}\nTime in seconds: {}\nWords typed: {},",
            self.wpm, self.accuracy, self.time_in_seconds, self.words_count
        )
    }
}

pub struct TypingGame {
    terminal: Term,
    terminal_columns: u16,
    keys_pressed_count: u64,
    errors_count: u64,

    cursor_x_position: usize,
    cursor_y_position: usize,
}

impl Default for TypingGame {
    fn default() -> TypingGame {
        let terminal = Term::stdout();
        let (_, terminal_cols) = terminal.size_checked().unwrap();

        return TypingGame {
            terminal,
            terminal_columns: terminal_cols,
            keys_pressed_count: 0,
            errors_count: 0,
            cursor_y_position: 0,
            cursor_x_position: 0,
        };
    }
}

impl TypingGame {
    pub fn start_game(&mut self, sentence: &str) -> Result<(), std::io::Error> {
        self.terminal.clear_screen()?;
        let mut words = get_sentence_words(sentence);
        println!("{}", sentence);

        let now = Instant::now();

        for word in words.iter_mut() {
            while !word.guessed {
                self.prompt_for_chars(word);
                word.set_guessed();
            }
        }

        self.print_result(words, now.elapsed().as_secs());
        Ok(())
    }

    fn prompt_for_chars(&mut self, game_word: &mut GameWord) {
        let mut chars = game_word.word.chars();
        let mut target_char = chars.next().unwrap();

        loop {
            self.set_cursor();

            let char = self.read_char();
            if char == target_char {
                println!("{}", style(char).green());
                self.move_cursor_position();

                target_char = match chars.next() {
                    Some(ch) => ch,
                    None => return,
                };
            } else {
                println!("{}", style(target_char).red());
                self.increment_error();
            }
        }
    }

    fn set_cursor(&self) {
        self.terminal
            .move_cursor_to(self.cursor_x_position, self.cursor_y_position)
            .unwrap();
    }

    fn move_cursor_position(&mut self) {
        self.cursor_x_position += 1;
        // Check if the y position reached the terminal column width and move to next cursor to
        // next row
        if self.cursor_y_position == self.terminal_columns as usize {
            self.cursor_y_position = 0;
            self.cursor_x_position += 1;
        }
    }

    fn increment_error(&mut self) {
        self.errors_count += 1;
    }

    fn read_char(&mut self) -> char {
        let char = self.terminal.read_char().unwrap();
        self.keys_pressed_count += 1;
        char
    }

    fn print_result(&self, words: Vec<GameWord>, time_in_seconds: u64) {
        let correct_chars: usize = words.into_iter().map(|word| word.word.len()).sum();

        let time_in_min = time_in_seconds as f32 / 60.0;
        let gross_wpm = (correct_chars as f32 / 5.0) / time_in_min;
        let net_wpm = (gross_wpm - (self.errors_count as f32 / time_in_min)) as u64;
        let accuracy = (correct_chars as f32 / self.keys_pressed_count as f32) * 100.0;

        println!(
            "Gross WPM: {:.0}\nNet WPM: {}\nAccuracy: {:.2}\nTime in seconds: {}",
            gross_wpm, net_wpm, accuracy, time_in_seconds,
        )
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
    guessed: bool,
}

impl GameWord<'_> {
    fn new(word: &str) -> GameWord {
        GameWord {
            word,
            guessed: false,
        }
    }

    fn set_guessed(&mut self) {
        self.guessed = true;
    }
}
