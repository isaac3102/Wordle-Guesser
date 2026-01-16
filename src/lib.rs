use std::env; 
use std::fs;
use std::hash::Hash; // file reader lib
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
    index: usize,
}

impl Character {
    pub fn new(character: char, placement: Placement, index: usize) -> Self {
        Character { character, placement, index }
    }

    fn update_placement(&mut self, new_placement: Placement, new_index: usize) {
        self.placement = new_placement;
        self.index = new_index;
    }

    fn get_character(&self) -> char {
        self.character
    }

    fn get_placement(&self) -> Placement {
        self.placement
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

fn set_state(character_list: &mut [Character;26], character: char, placement: Placement, index: usize) {
    for char_struct in character_list.iter_mut() {
        if char_struct.get_character() == character {
            char_struct.update_placement(placement, index);
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
    let contents = fs::read_to_string("5_letters.txt")?;

    Ok(contents)
}

// analyze_contents function to analyze character frequency and ranking
pub fn analyze_contents(data: Arc<String>) -> [Character; 26] {

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
    println!("{:?}", char_count);
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
    let mut character_list: [Character;26] = [Character::new(' ', Placement::Unknown, 0); 26];
    for index in 0..26 {
        character_list[index] = Character::new((char_ranking[index] + b'a') as char, Placement::Unknown, 0);
        
    }
    return character_list;

}

// main_selector function to select main characters (currently a placeholder)
pub fn main_selector(data: Arc<String>, ranking: Arc<[Character; 26]>, answer: String) {
    
    /* 
    
    Takes in an answer string that will be used to check the correctness of guesses.

    two arrays:
    choice: [Character; 26] - represents all possible characters (a-z) with their placement status and index.
    word: [Character; 5] - represents the current guessed word with its characters, placement status, and index.


     */
    let mut character_list: [Character;26] = *ranking.clone();
    let mut word: [Character;5] = [Character::new(' ', Placement::Unknown, 0); 5];

    set_state(&mut character_list, 'a', Placement::Absent, 0);
    set_state(&mut character_list, 'l', Placement::Absent, 2);
    set_state(&mut character_list, 's', Placement::Absent, 4);
    set_state(&mut character_list, 'b', Placement::Absent, 4);
    set_state(&mut character_list, 'n', Placement::Absent, 4);
    set_state(&mut character_list, 't', Placement::Absent, 4);
    set_state(&mut character_list, 'c', Placement::Absent, 4);
    set_state(&mut character_list, 'd', Placement::Absent, 0);
    set_state(&mut character_list, 'v', Placement::Absent, 0);

    match form_word(data.clone(), character_list.clone(), [Character::new('_', Placement::Unknown, 0),
    Character::new('_', Placement::Unknown, 1), Character::new('e', Placement::Correct, 2),
    Character::new('r', Placement::Correct, 3), Character::new('y', Placement::Correct, 4)]) {
        Some(word) => println!("Formed word: {}", word),
        None => println!("No word could be formed with the given characters."),
    }   

}

// given a list of specific characters and their placements, form a word
fn form_word(data: Arc<String>, character_list: [Character;26], letters: [Character;5]) -> Option<String> {
    let mut formed_word: Vec<u8> = vec![b'_'; 5]; 
    let mut checked:u8 = 0;
    let mut full:bool = true;
    for word in data.lines() {
        println!("Checking word: {}", word);
        for char in &letters {
            // check first for correct placements
            if char.get_placement() == Placement::Correct {
                match position_locator(word.to_string(), char.get_character()) {
                    Some(c) => {
                        if c.get_index() != char.get_index() { // if index does not match, break
                            full = true;
                            checked = 0;
                            formed_word = vec![b'_'; 5];
                            break;
                        } else {
                            formed_word[char.get_index() as usize] = char.get_character() as u8;
                            checked += 1;
                        }
                    },
                    None => {
                        full = true;
                        checked = 0;
                        formed_word = vec![b'_'; 5];
                        break;
                    },
                }
            } 
            else if char.get_placement() == Placement::Present {
                match position_locator(word.to_string(), char.get_character()) {
                    Some(c) => {
                        if c.get_index() != char.get_index() { // if index does not match, allow
                            checked += 1;
                        } else {
                            full = true;
                            checked = 0;
                            formed_word = vec![b'_'; 5];
                            break;
                        }
                    },
                    None => {
                        full = true;
                        checked = 0;
                        formed_word = vec![b'_'; 5];
                        break;
                    },
                }
            }
            else { // if its not correct, check if its inside
                if char.get_character() == '_' {
                    checked += 1;
                    full = false;
                    continue;
                } else {
                    match position_locator(word.to_string(), char.get_character()){
                    Some(c) => {
                        println!("Found character {} at index {}", char.get_character(), c.get_index());
                        println!("formed_word before: {:?}", String::from_utf8(formed_word.clone()).unwrap());
                        if formed_word[c.get_index() as usize] == b'_' { // if empty
                            formed_word[c.get_index() as usize] = char.get_character() as u8;
                            checked += 1;
                        } 
                        else if formed_word[c.get_index() as usize] == char.get_character() as u8 {
                            formed_word[c.get_index() as usize + 1] = char.get_character() as u8;
                            checked += 1;

                        }
                        else {
                            full = true;
                            checked = 0;
                            formed_word = vec![b'_'; 5];
                            break;
                        }                  
                    
                      },
                    None => {
                        full = true;
                        checked = 0;
                        formed_word = vec![b'_'; 5];
                        break;
                }
            }
                }

                    
                }
                

        
            
        }

        if checked == 5 {
            if full == true {
                return Some(String::from_utf8(formed_word).unwrap());
            }
            else {
                let mut skip = false;
                for i in word.chars() {
                    if get_state(&character_list, i) == Some(Placement::Absent) {   
                        println!("Character {} is marked absent, skipping word {}", i, word);
                        skip = true;
                        continue;
                    }
                } if skip != true {
                    return Some(word.to_string());
                } else {
                    full = true;
                        checked = 0;
                        formed_word = vec![b'_'; 5];
                
                }     
                
            }
        }
    
    }
    return None;
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
            return Some(Character::new(c, Placement::Present, i));
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
            if (ranking[count].get_placement() == Placement::Absent) {
                continue;
            }
            let mut new_permutation: String = permutation.clone();
            new_permutation.push((ranking[count].get_character() as char));
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
            if (ranking[count].get_placement() == Placement::Absent) {
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

