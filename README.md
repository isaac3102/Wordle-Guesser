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
    - how often a character appears
    - whether a character is absent from the word
    - whether a present character's position is known
    - whether a present character's incorrect position is known

With the information above, we can assign a value to each character.

> Assigned Values
>> Absent Characters: -1.0
>> Correct Characters (Present and Known Position): 0.0
>> Unknown and Present (Unknown Position): 0.0 - 1.0
>> *Further explanation under function assign_weightage()*


Each word is then given a value based on the values of their characters
> *This makes it so that words are not completely ruled out of being suggested but instead ***less valuable*** to be used*

However, some words consist of **unique** permutations of characters as such before suggesting a word based on its value, we attempt to form a word with known characters.

If a **single word** is formed, then its most likely the correct answer.
Otherwise, we carry on and suggest the word that has the **highest value**.

## Functions related to main logic

**main_selector()**
> This function is the kickoff point of the algorithm
> Takes in a list of words and the answer
> Code Flow:
>> 1. count the number of times a character appears  
>> 2. revise whether a character is known or absent
>> 3. assign value to characters
>> 4. attempt to form a word
>> 5. if able to form a word, suggest it
>> 6. else, take word with the highest value
>> 7. repeat until

**char_appearances()**
>  

**revise_placement()**
>

**assign_weightage()**
>

**provide_suggestion()**
>

**check_answer()**
>

## Helper Functions

**read_file()**




## Unused Functions

### Process Journal

