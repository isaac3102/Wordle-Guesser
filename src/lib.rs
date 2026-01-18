use std::env; 
use std::fs;
use std::hash::Hash;
use std::vec; // file reader lib
use anyhow::{anyhow, Result};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Placement {
    Unknown, // Not yet determined
    Absent, // Not in the word
    Present, // In the word
    Correct, // Correct position
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Character {
    character: char,
    placement: Placement,
    weightage: f32,
}

impl Character {
    pub fn new(character: char, placement: Placement) -> Self {
        Character { character, placement, weightage: 0.0 }
    }

    fn update_placement(&mut self, new_placement: Placement) {
        self.placement = new_placement;
    }

    fn update_weightage(&mut self, new_weightage: f32) {
        self.weightage = new_weightage;
    }

    fn get_character(&self) -> char {
        self.character
    }

    fn get_placement(&self) -> Placement {
        self.placement
    }

    fn get_weightage(&self) -> f32 {
        self.weightage
    }

}

fn set_state(character_list: &mut [Character;26], character: char, placement: Placement) {
    for char_struct in character_list.iter_mut() {
        if char_struct.get_character() == character {
            char_struct.update_placement(placement);
            break;
        }
    }
}

fn get_state(character_list: &[Character;26], character: char) -> Option<Placement> {
    for char_struct in character_list.iter() {
        if char_struct.get_character() == character {
            return Some(char_struct.get_placement());
        }
    }
    None
}

fn get_weightage(character_list: &[Character;26], character: char) -> Option<f32> {
    for char_struct in character_list.iter() {
        if char_struct.get_character() == character {
            return Some(char_struct.get_weightage());
        }
    }
    None
}

/* Usage of lib.rs

read_file -> read files containing words, split it and collect into a vector (list)
e.g. [hello, allow, ..]

analyze_contents -> analyze content
statistical numbers would include:
number of appearances,

*/

/*
Notes:



*/
// read_file function to read contents of a file
pub fn read_file() -> Result<String> {
    let contents = fs::read_to_string("words.txt")?;

    Ok(contents)
}

// not used currently
// analyze_contents function to analyze character frequency and ranking
pub fn analyze_contents(data: Arc<String>) -> [u8; 26] {

    // b'a' -> b means byte value of 'a'

    let mut char_count: [u16; 26] = [0; 26];
    let mut char_ranking: [u8; 26] = [0; 26];

    for i in 0..26 {
        char_ranking[i as usize] = i;
    }
    /* Loop through each character */
    for line in data.lines() {
        for character in line.chars() {
            if character >= 'a' && character <= 'z' {
                let idx = (character as u8 - b'a') as usize;
                char_count[idx] += 1;
            }
            // (character as u8) % 97
            // as u8 converts it to unicode (as turns primitive types to other primitive types)
           
        }
    }
    for count in 0..26 {
        let mut principal:u16 = char_count[count];
        let mut swap = count;
        for check in count..26 {
            if principal < char_count[check] {
                principal = char_count[check];
                swap = check;
            }
        }
        if swap != count {
            char_ranking.swap(count, swap);
            char_count.swap(count, swap);
        }
    }
    return char_ranking;

}

// create list of characters based on placement
fn create_list(placement: Placement, character_list: [Character;26]) -> Vec<char> {
    let mut results: Vec<char> = Vec::new();
    for char_struct in character_list.iter() {
        if char_struct.get_placement() == placement {
            results.push(char_struct.get_character());
        }
    } return results;
}

// count character appearances in remaining possible words
fn char_appearances(data:Arc<String>, character_list: [Character;26], formed_word: [char;5], attempted_characters: HashMap<char, Vec<usize>>) -> HashMap<char, u16> {
    let mut results: HashMap<char, u16> = HashMap::new();
    let absent_chars = create_list(Placement::Absent, character_list.clone());
    let correct_chars = create_list(Placement::Correct, character_list.clone());
    let present_chars = create_list(Placement::Present, character_list.clone());
    let mut skip: bool = false;
    for word in data.lines() {
        if word.chars().any(|c| absent_chars.contains(&c)) {
            continue;
        } else if correct_chars.len() > 0 {
            for (i, c) in formed_word.iter().enumerate() {
                if *c != '_' { // if its not an underscore
                    if word.chars().nth(i).unwrap() != *c { // then check if it matches the position, if it doesnt skip the word
                        skip = true;
                        break;
                    } 
                }
            }
        } else if present_chars.len() > 0 {
            for c in present_chars.iter() {
                if let Some(position_list) = attempted_characters.get(c) {
                    for pos in position_list.iter() {
                        if let Some(ch) = word.chars().nth(*pos) {
                            if ch == *c {
                                skip = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    
        if skip == true {
            skip = false;
            continue;
        } else {
             for c in word.chars() {
                *results.entry(c).or_insert(0) += 1;
            }
        }
           
        }
        
     return results;
}

// main_selector function to select main characters (currently a placeholder)
pub fn main_selector(data: Arc<String>, answer: String) {
    
    let mut character_list: [Character;26] = [Character::new('a', Placement::Unknown); 26];
    for i in 0..26 {
        character_list[i as usize] = Character::new((i + b'a') as char, Placement::Unknown);
    }
    let mut formed_word: [char;5] = ['_';5];
    let mut attempted_characters: HashMap<char, Vec<usize>> = HashMap::new();

    let mut counter = 0;

    while formed_word != answer.chars().collect::<Vec<char>>()[..5] {
        let appearances = char_appearances(data.clone(), character_list.clone(), formed_word.clone(), attempted_characters.clone());
        revise_placement(&mut character_list, appearances.clone());

        assign_weightage(&mut character_list, appearances);

        let first_suggestion = form_word(data.clone(), character_list.clone(), attempted_characters.clone(), formed_word.clone()).unwrap();

        if first_suggestion.len() == 1 {
            println!("Only one character left to form the word.");
            check_answer(first_suggestion[0].clone(), &mut formed_word, &mut character_list, &mut attempted_characters, answer.clone());
        } else {
            println!("Multiple possible words can be formed.");
            let suggestions = provide_suggestions(data.clone(), character_list.clone(), &mut attempted_characters);
            let weighted_suggestion = suggestions.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0.clone();
           println!("Suggested word: {}", weighted_suggestion);
            check_answer(weighted_suggestion, &mut formed_word, &mut character_list, &mut attempted_characters, answer.clone());
        }
        counter += 1;
    }

    println!("Final formed word: {:?}", String::from_iter(formed_word));
    println!("Solved the Wordle in {} attempts!", counter);

}

fn check_answer(attempt: String, formed_word: &mut [char;5], character_list: &mut [Character;26], attempted_characters: &mut HashMap<char, Vec<usize>>, answer: String){
    for (i, c) in attempt.chars().enumerate() {
        if answer.contains(c) {
            if answer.chars().nth(i).unwrap() == c {
                 set_state(character_list, c, Placement::Correct);
                formed_word[i] = c;
            } else {
                set_state(character_list, c, Placement::Present);
                attempted_characters.entry(c).or_insert(Vec::new()).push(i);

            }
        }
         else {
            set_state(character_list, c, Placement::Absent);
        }
    }
}

fn revise_placement(character_list: &mut [Character;26], appearances: HashMap<char, u16>) {
    for char_struct in character_list.iter_mut() {
        let count = appearances.get(&char_struct.get_character()).unwrap_or(&0);
            if count == &0 {
                char_struct.update_placement(Placement::Absent);
    }
}
}

// takes in list of character and their appearances, 
fn assign_weightage(character_list: &mut [Character;26], appearances: HashMap<char, u16>) {
    let min = appearances.values().min().unwrap_or(&1);
    let max = appearances.values().max().unwrap_or(&1);
    for char_struct in character_list.iter_mut() {
        if char_struct.get_placement() == Placement::Absent {
            char_struct.update_weightage(-1.0);
        } else if char_struct.get_placement() == Placement::Correct {
            char_struct.update_weightage(0.0);
        } 
        else {
                let count = appearances.get(&char_struct.get_character()).unwrap_or(&0);
                let weightage = (*count as f32 - *min as f32) / (*max as f32 - *min as f32);
                char_struct.update_weightage(weightage);
            }
        }
    }


/*

Based on the character list and their weightages, assign values to the words in the data and provide suggestions accordingly.

Logic:

An absent character will have a value of -1.0
A correct character will have a value of 0.0 (since we already know it is correct and dont need to suggest words with it)
A present character will have a value between 0.0 and 1.0 based on its frequency in the remaining possible words.
A unknown character will have a value between 0.0 and 1.0 based on its frequency in the remaining possible words.

Any word that contains duplicate characters e.g. hello will only count the weightage of each character once.
*/
fn provide_suggestions(data: Arc<String>, character_list: [Character;26], attempted_characters: &mut HashMap<char, Vec<usize>>) -> HashMap<String, f32> {
    let mut suggestions: HashMap<String, f32> = HashMap::new();
    let present_chars = create_list(Placement::Present, character_list.clone());
    for word in data.lines() {
        let mut in_word: Vec<char> = Vec::new(); // to track characters that are in the word
        let mut value_of_word: f32 = 0.0;
        for (i, c) in word.chars().enumerate() { // for each character in the word
            if in_word.contains(&c) {
                continue; // skip if already counted
            } else {
                if present_chars.contains(&c) { // if the character is present, check if the position has been attempted before
                    // get all attempted positions for this character
                    for pos in attempted_characters.get(&c).unwrap_or(&Vec::new()).iter() {
                        if *pos == i {
                            continue; // skip if position matches attempted position
                        }}
                } else {
                    if let Some(weightage) = get_weightage(&character_list, c) {
                    value_of_word += weightage;
                    }
                }
                
            }
            
            in_word.push(c); // add character to in_word list
        }
        suggestions.insert(word.to_string(), value_of_word);
        }
        suggestions
    }
    


// given a list of specific characters and their placements, form a word
fn form_word(data: Arc<String>, character_list: [Character;26], attempted: HashMap<char, Vec<usize>>, formed_word: [char;5]) -> Option<Vec<String>> {
    let mut suggestions: Vec<String> = Vec::new();
    let present_chars = create_list(Placement::Present, character_list.clone());
    let correct_chars = create_list(Placement::Correct, character_list.clone());
    let absent_chars = create_list(Placement::Absent, character_list.clone());
    for word in data.lines() {
        if word.chars().any(|c| absent_chars.contains(&c)) {
            continue;
        } else {
            let mut skip_word = false;
            // check first for correct placements
            if correct_chars.len() > 0 {
                for (i, c) in formed_word.iter().enumerate() {
                    if *c != '_' { // if not blank
                        if word.chars().nth(i).unwrap() != *c { // if doesnt match, dont use this word
                            skip_word = true;
                            break;
                        } 
                    }
                }
            } 
            // then check for present placements and ignore words that had them in the old position
            if !skip_word && present_chars.len() > 0 {
                for c in present_chars.iter() {
                    // store old positions in position_list then grab character in possible word at those positions
                    if let Some(position_list) = attempted.get(c) {
                        for pos in position_list.iter() {
                            if let Some(ch) = word.chars().nth(*pos) {
                                if ch == *c { // if it is then skip this word
                                    skip_word = true;
                                    break;
                                }
                            }
                        }
                    }
                    if skip_word {
                        break;
                    }
                }
            }
            // by this point, if code has not set skip_word, then its a possible word
            if !skip_word {
                suggestions.push(word.to_string());
            }
           }
    }
    return Some(suggestions);
}

// entry_pattern function to analyze patterns based on initial character
pub fn entry_pattern(data: Arc<String>, ranking: [Character; 26], initial: usize, limiter: usize) -> Result<HashMap<String, u16>>{
    let permutation = ranking[initial].get_character().to_string(); 
    let results = analyze_patterns(data, ranking, &mut permutation.to_string(), limiter).ok_or_else(|| anyhow!("Nothing Found"))?;
    let max_entry = results.iter().max_by_key(|&(_, count)| count);
    println!("Max Entry for initial {}: {:?}", permutation, max_entry);
    for (word, count) in &results {
        if is_unique(word.to_string()) {
            if max_entry.is_some() && word.contains(max_entry.unwrap().0) {
                println!("Unique word: {} with count {}", word, count);
            }
        }
    }
    Ok(results)
}

fn position_locator(word: String, character: char) -> Option<Character> {
    for (i, c) in word.chars().enumerate() {
        if c == character {
            return Some(Character::new(c, Placement::Present));
        }
    } return None;
}

fn is_unique(word: String) -> bool {
    let mut temp: Vec<char> = Vec::new();
    for i in word.chars() {
        if temp.contains(&i) {
            return false;
        } else {
            temp.push(i);
        }
    } return true;
}

// recursive function to analyze patterns
fn analyze_patterns(data: Arc<String>, ranking: [Character;26], permutation: &mut String, limiter: usize) -> Option<HashMap<String, u16>>{

    let mut results = HashMap::new();

    // base case
    if permutation.len() == 1{
        for count in 0..limiter {
            if ranking[count].get_placement() == Placement::Absent {
                continue;
            }
            let mut new_permutation: String = permutation.clone();
            new_permutation.push(ranking[count].get_character() as char);
            match analyze_patterns(data.clone(), ranking.clone(), &mut new_permutation, limiter.clone()) {
                Some(res) => {
                    results.extend(res);
                },
                None => {
                    continue;
                },
            }
        }

        return Some(results);
        
    }

    // check appearances of permutation in data
    let mut appearances: u16 = 0;
    for word in data.lines() {
        if word.contains(&*permutation) {
            appearances += 1;
        }
    }

    // base cases
    if appearances == 0 {
        return None
    } else if appearances == 1 {
        return Some(HashMap::from([(permutation.clone(), appearances)]));
    } 
    else {
        for count in 0..limiter {
            if ranking[count].get_placement() == Placement::Absent {
                continue;
            }
            results.insert(permutation.clone(), appearances); 
            let mut new_permutation: String = permutation.clone();
            new_permutation.push(ranking[count].get_character() as char);
            match analyze_patterns(data.clone(), ranking.clone(), &mut new_permutation, limiter.clone()) {
                Some(res) => {
                    results.extend(res);
                },
                None => {
                    continue;
                },
            }
        }
        return Some(results);
    }
    

}

