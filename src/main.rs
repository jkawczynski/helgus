mod game;
use game::TypingGame;

mod words;
use words::download_words;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let words = download_words().await?;

    let sentence = words[0..60].join(" ");

    TypingGame::default()
        .start_game(sentence)
        .expect("Error starting the game");

    Ok(())
}
