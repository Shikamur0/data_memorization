use chrono::Utc;
use chrono::Datelike;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
struct WordList {
    words: Vec<String>,
}

fn load_words_from_json<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let word_list: WordList = serde_json::from_reader(reader)?;
    Ok(word_list.words)
}

fn get_daily_words(words: &[String], count: usize) -> Vec<String> {
    let today = Utc::now().date_naive().ordinal();
    let mut rng = rand::rngs::StdRng::seed_from_u64(today as u64);
    let mut selected_words = words.to_vec();
    selected_words.shuffle(&mut rng);
    selected_words.into_iter().take(count).collect()
}

fn main() {
    // Загружаем слова из JSON файла
    let words = match load_words_from_json("words.json") {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Ошибка при загрузке слов: {}", e);
            return;
        }
    };

    // Получаем слова дня
    let daily_words = get_daily_words(&words, 5);
    
    // Выводим слова
    println!("Слова дня: {:?}", daily_words);
}
