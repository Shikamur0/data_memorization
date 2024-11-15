use chrono::Utc;
use chrono::Datelike;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io;

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
    let total_words = words.len();
    
    // Используем остаток от деления, чтобы избежать выхода за пределы массива
    let start_index = ((today - 1) as usize) % total_words;

    // Создаем вектор для выбранных слов
    let mut selected_words = Vec::new();

    // Заполняем вектор выбранными словами в зависимости от текущего дня
    for i in 0..count {
        let index = (start_index + i) % total_words; // Индекс слов с учетом зацикливания
        selected_words.push(words[index].clone());
    }

    selected_words
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

    println!("Нажмите Enter, чтобы завершить процесс...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
