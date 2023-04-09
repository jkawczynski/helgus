#[derive(serde::Deserialize)]
struct Words {
    english1000: Vec<String>,
}
pub async fn download_words() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get("https://typings.gg/texts/random.json")
        .send()
        .await?;

    println!("Downloading words...");

    let words = response.json::<Words>().await?;
    Ok(words.english1000)
}
