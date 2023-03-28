mod game;
use game::TypingGame;

fn main() {
    //let sentence = "This is a new test, beat this game as fast as possible";
    let sentence = "Lorem ipsum is placeholder text commonly used in the graphic, print, and publishing industries for previewing layouts and visual mockups.";

    TypingGame::default()
        .start_game(sentence)
        .expect("Error starting the game");
}
