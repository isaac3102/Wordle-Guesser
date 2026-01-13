use std::env; 
use std::fs; // file reader lib
use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;

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

pub fn read_file() -> Result<String> {
    let contents = fs::read_to_string("5_letters.txt")?;

    Ok(contents)
}

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

pub fn entry_pattern(data: Arc<String>, ranking: Arc<[u8; 26]>, start: u8) {
    analyze_patterns(data, ranking, start.to_string());
}

fn analyze_patterns(data: Arc<String>, ranking: Arc<[u8; 26]>, permutation: String){
    let mut appearances: u16 = 0;

    

}

