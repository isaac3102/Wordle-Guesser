use crate::main_selector;
use crate::read_file;

pub fn test_words() {
    let content = read_file().unwrap();
    let mut total_attempts = 0;
    for word in content.lines() {
        total_attempts += main_selector(content.clone(), word.to_string());
    }
    println!("Average attempts: {}", total_attempts as f32 / content.lines().count() as f32);
}