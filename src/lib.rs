use std::env; 
use std::fs; // file reader lib
use anyhow::Result;

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

pub fn analyze_contents(data: String){
    
    let mut char_count: [u16; 26] = [0; 26];
    let mut char_ranking: [u8; 26] = [0; 26];
    let mut probability: [[u16; 26];5] = [[0;26];5];

    for i in 0..26 {
        char_ranking[i as usize] = i;
    }
    /* Loop through each character */
    for line in data.lines() {
        for character in line.chars() {
            // (character as u8) % 97
            // as u8 converts it to unicode (as turns primitive types to other primitive types)
            println!("{} as {}", character, (character as u8) % 97);
            char_count[((character as u8) % 97) as usize] += 1;
        }
    }
    println!("{:?}", char_count);
    for count in 0..26 {
        let mut principal:u16 = char_count[count as usize];
        let mut swap = count;
        for check in count..26 {
            if principal < char_count[check as usize] {
                principal = char_count[check as usize];
                swap = check;
            }
        }
        if swap != count {
            char_ranking.swap(count, swap);
            char_count.swap(count, swap);
        }
    }
    println!("{:?}", char_ranking);
    println!("{:?}", char_count);
}

fn read_past() { // For Later
}

