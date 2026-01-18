# Wordle-Guesser
Suggest words until the wordle is solved.

## Background
Wordle is a daily guessing game in which users will have to try and guess a 5 letter word with a limited amount of tries. 

## Data
The algorithm uses a .txt file that contains 5 letter words used in the game Wordle.
> file_name: words.txt

## Algorithm
*character refers to letters*

The algorithm solves the wordle by using information based on:

-   how often a character appears
-   whether a character is absent from the word (*Absent*)
-   whether a present character's position is known (*Correct*)
-   whether a present character's incorrect position is known (*present*)

With the information above, we can assign a value to each character.

> - Absent Characters: -1.0
> - Correct Characters (Present and Known Position): 0.0
> - Unknown and Present (Unknown Position): 0.0 - 1.0
> - *Further explanation under function assign_weightage()*


Each word is then given a value based on the values of their characters
> *This makes it so that words are not completely ruled out of being suggested but instead ***less valuable*** to be used*

However, some words consist of **unique** permutations of characters - as such before suggesting a word based on its value, we attempt to form a word with known characters.

If a **single word** is formed, then its most likely the correct answer.
Otherwise, we carry on and suggest the word that has the **highest value**.

## Data Types

**Character**

Custom struct that holds the following:
- character: Char
- placement: Placement
- weightage: f32

**Placement**

Custom enum that can be:
- Unknown
- Absent
- Present
- Correct

The Character struct is used to hold more information related to a single letter, identifying whether it is placement in the final word is any of the following: (Absent, Present, Correct).

Additionally, it is used to hold its value (weightage) which will be used to determine the value of any given word.

## Main Functions

**main_selector()**

This function is the kickoff point of the algorithm

Takes in a list of words and the answer and attempts to guess the word

Code Flow:
> 1.    count the number of times a character appears  
> 2.    revise whether a character is known or absent
> 3.    assign value to characters
> 4.    attempt to form a word
> 5.    if able to form a word, suggest it
> 6.    else, take word with the highest value
> 7.    repeat until



**char_appearances()**

This function counts the number of times a character appears in a word

Takes in a list of words, list of characters, word that has been formed so far and a list of present characters and their positions

> Returns a HashMap of (characters and the number of times they appear)

Code Flow:
> 1.    Create a list of absent, correct and present chars
> 2.    Skip a word if it contains an absent character, a present character in the wrong position and if a known character is in the wrong position
> 3.    Otherwise, count the characters that appear in the word



**revise_placement()**

This function revises whether a character is present or absent
***It should be used after the number of times a character appears has been counted***

> Modifies the placement of characters in the given list of Characters

Code Flow:
> 1.    Loop through Characters in the given list
> 2.    If there's no count of the character then mark it as absent

*As words that contain present characters in the wrong position and absent characters are ignored characters that aren't counted will most likely be absent*



**assign_weightage()**

This function assigns a value to the character based on the number of times it will appear

> Modifies the values of characters in the given list of Characters

Code Flow:
> 1.    Assign a value of -1.0 if the character is not inside the word (Absent)
> 2.    Assign a value of 0.0 if the character is already in the correct place (Correct)
> 3.    Otherwise, assign a normalized value of 0.0 - 1.0 (normalized with min-max) 



**provide_suggestion()**

This function generates a HashMap of (word: value) to provide suggestions on what the next best word is

> Returns a HashMap of (word: value)

Code Flow:
> 1.    Loop through each word in the given word list and sum up the value of all unique characters



**check_answer()**

This function modifies any given list of Characters

> Modifies the placement of characters based on the attempted answer and attempted list of positions

Code Flow:
> 1.    Checks whether the answer contains any characters in the attempted answer
> 2.    If it does, check whether it is in the correct position and set its placement as Correct
> 3.    Otherwise, set it as Present and store its index as an attempt
> 4.    If the answer does not contain the character, set its placement as Absent

## Helper Functions
Smaller functions used to help the Main Functions work


**read_file()**
: This function reads any given text file and stores it as a String

**create_list()**
: Creates a list of Characters based on whether its Absent, Present or Correct

**set_weightage()**
: Set value of any given Character in a given List of Characters

**set_state()**
: Set placement of any given Character in a given List of Characters


## Unused Functions
Functions that I created thinking that it would be part of the solving process

position_locator(), is_unique(), entry_patterns(), analyze_patterns(), analyze_contents()


## Process Journal

**Beginning of Project**

Beginning the project, I thought that I should first look at the occurence rate and probability of letters that could appear in the word, as such I first began with making the analyze_contents() function which counts the number of times a letter appears.

Since, that would mean that a word containing common letters would eliminate more words if the letter happens to be absent.

Secondly, as words aren't random and patterns do emerge, I wanted to analyze common patterns, as such I created the entry_patterns() and analyze_patterns() function. However, I did not get any where after that.

**Update 1**

Created the Character *struct* and Placement *enum*, to keep track of letter information. At first, each Character held an index value that would tell the function where it should be if its Correct.

However, I very soon realised that it was not very smart as some words contain two of the same letters. e.g. 'hello'.


**Update 2**

Shifted to a value-based suggestion system, based on the strategy for wordle which is to eliminate letters so that it would be easier to form a word from the remaining letters. As such, I did not want the algorithm to tunnel vision when it knows some Correct characters. Using a value-based system I'll be able to select the most valuable word to play next.

I realised this method was also not that fool-proof, sometimes some combinations of letters will result in a single word. Therefore, before I try and create a list of suggested words, I created a function to attempt to form a word. If a single word comes out, then it'll use that word.

In this update, I can safely say that the algorithm can guess the word. However, it can be improved, I'm thinking of implementing a pattern recognizer together with forming the word/suggesting the word.

note: After fiddling around with it, I realised that the form_word function was not properly forming words (it made words that did not have present words inside)

note: Encountered another issue, where max == min and it resulted in NaN. Changed so that it will assign a value of 1.0 if it happens, additionally handled errors for weighted_suggestions

note: Final change would be to add graphical UI and testing mechanisms