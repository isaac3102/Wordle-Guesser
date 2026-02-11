use wordle_guesser::{Character, Placement, 
    read_file, char_appearances, revise_placement, assign_weightage, provide_suggestions, form_word, main_selector,
set_state, get_state, get_weightage};
use std::collections::HashMap;
use thirtyfour::prelude::*;
use thirtyfour::Key; 
use tokio::time::Duration;
use tokio::signal;
use std::error::Error;
use std::process::Command;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let content = read_file().unwrap();

    let mut character_list: [Character;26] = [Character::new('a', Placement::Unknown); 26];
    for i in 0..26 {
        character_list[i as usize] = Character::new((i + b'a') as char, Placement::Unknown);
    }
    let mut formed_word: [char;5] = ['_';5];
    let mut attempted_characters: HashMap<char, Vec<usize>> = HashMap::new();

    let solved = false;

    // Run chromedriver
    let chromedriver = Command::new("./lib/chromedriver.exe")
        .arg("--port=9515").spawn()?;

    // make initial connection to webdriver
    let caps = DesiredCapabilities::chrome();
    // Create struct focused on chrome, providing useful specific helpers
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to specified page!
    driver.goto("https://www.nytimes.com/games/wordle/index.html").await?;

    // Note query waits until element appears
    // Begin 'playing'
    driver.query(By::Css("[data-testid='Play']")).first().await?.click().await?;

    driver.query(By::Css("[aria-label='Close']")).first().await?.click().await?;

    let mut counter = 1;
    // Interface should be open here now
    loop {
        println!("on loop {}", counter);
        let mut suggested: String = String::new(); // create new variable to hold suggested word
        // count appearance
        let appearances = char_appearances(content.clone(), character_list.clone(), formed_word.clone(), attempted_characters.clone());
        revise_placement(&mut character_list, appearances.clone());
        assign_weightage(&mut character_list, appearances);
        let first_suggestion = form_word(content.clone(), character_list.clone(), attempted_characters.clone(), formed_word.clone()).unwrap();

        // if im unable to form a word with what I know (which means a definite answer...)
        if first_suggestion.len() == 1 {
            suggested = first_suggestion[0].clone();
        } else { // Then i choose based on best possible answer!
            let suggestions = provide_suggestions(content.clone(), character_list.clone(), &mut attempted_characters);
            //total_cmp dont care abt NaN but my value doesnt have so its fine
            suggested = suggestions.iter().max_by(|a, b| a.1.total_cmp(b.1)).expect("No suggestions found!").0.to_string();
        }

        println!("attempting {:?}", suggested);

        // each row is labelled as aria-label="Row {1-6}"
        let row_number = format!("[aria-label='Row {}']", counter);
        let row = driver.query(By::Css(row_number)).first().await?;

        // Loop of sending and ensuring its there
        for (i, letter) in suggested.chars().enumerate() {
            loop {
                println!("trying {:?}", letter);
                let selector = format!("[data-key='{}']", letter);
                let letter_button = driver.query(By::Css(&selector))
                .first().await?;
                
                letter_button.wait_until().clickable().await?;
                letter_button.click().await?;

                let selector = format!("[aria-label^='{}']", (i+1));
                let letter_box = row.query(By::Css(selector)).first().await?;

                if let Some(state) = letter_box.attr("data-state").await? {
                    if state.contains("tbd") {
                        break;
                    }
                }
            }

        }

    driver.find(By::Css("body")).await?.send_keys(Key::Enter).await?;



    // After grabbing the row, loop through each box
    for i in 1..6 {
        // *= contains
        // ^= starts with
        // $= ends with
        let selector = format!("[aria-label^='{}']", i);
        let letter_box = row.query(By::Css(selector)).first().await?;
        let letter = suggested.chars().nth(i-1).expect("None");

        // poll until ready to receive data
        loop {
            if let Some(state) = letter_box.attr("data-state").await? {
                if state.contains("correct") || state.contains("present") || state.contains("absent") {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

            match letter_box.attr("data-state").await? {
                Some(position) => {
                    println!("Checking {} in placement {}", letter, position);
                    if position == "correct" {
                        set_state(&mut character_list, letter, Placement::Correct);
                        formed_word[i-1] = letter;
                    } else if position == "present" {
                        set_state(&mut character_list, letter, Placement::Present);
                        attempted_characters.entry(letter).or_insert(Vec::new()).push(i-1);
                    } else if position == "absent" {
                        // assign absent only if unknown position
                        if let Some(placement) = get_state(&mut character_list, letter) {
                            if placement == Placement::Unknown {
                                set_state(&mut character_list, letter, Placement::Absent);
                            }
                        }
                    }
                }
                None => {
                    println!("Nothing...");
                }
            }
        }

        if !formed_word.contains(&'_') {
            break;
        }

        counter+=1;

    }
    
    let _ = driver.leak();

    Ok(())
}