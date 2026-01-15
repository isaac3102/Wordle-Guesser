use std::env; 
use std::fs;
use std::hash::Hash; // file reader lib
use anyhow::{anyhow, Result};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Placement {
    Absent, // Not in the word
    Present, // In the word but wrong position
    Correct, // In the correct position
    Unknown, // Not yet analyzed
}

#[derive(Copy, Clone)]
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
    for index in 0..26 {
        println!("{:?} appeared {} times", (char_ranking[index] + b'a') as char, char_count[index]);
    }
    return char_ranking;

}

pub fn main_selector(found_patterns: HashMap<String, u16>) {
    let mut choice: [Character;26] = [Character::new(' ', Placement::Unknown, 0); 26]; // main choice array
    for i in 0..26 {
        choice[i] = Character::new((i as u8 + b'a') as char, Placement::Unknown, 0);
    }
}

// entry_pattern function to analyze patterns based on initial character
pub fn entry_pattern(data: Arc<String>, ranking: Arc<[u8; 26]>, initial: usize) -> Result<HashMap<String, u16>>{
    let permutation = (ranking[initial] + b'a') as char; 
    let results = analyze_patterns(data, ranking, &mut permutation.to_string()).ok_or_else(|| anyhow!("Nothing Found"))?;
    Ok(results)
}

// recursive function to analyze patterns
fn analyze_patterns(data: Arc<String>, ranking: Arc<[u8; 26]>, permutation: &mut String) -> Option<HashMap<String, u16>>{

    let mut results = HashMap::new();

    // base case
    if permutation.len() == 1{
        for count in 0..26 {
            let mut new_permutation: String = permutation.clone();
            new_permutation.push((ranking[count] + b'a') as char);
            match analyze_patterns(data.clone(), ranking.clone(), &mut new_permutation) {
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
        for count in 0..26 {
            results.insert(permutation.clone(), appearances); 
            let mut new_permutation: String = permutation.clone();
            new_permutation.push((ranking[count] + b'a') as char);
            match analyze_patterns(data.clone(), ranking.clone(), &mut new_permutation) {
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

