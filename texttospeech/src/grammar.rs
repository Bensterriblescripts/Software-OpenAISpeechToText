use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn check_spelling(response: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let words: Vec<&str> = response.split_whitespace().collect();
    let parsed_response = "";

    // Loop through all words in the response
    for (index, word) in words.iter().enumerate() {

        // Move on if this is our first word -- Do more with this later
        if index == 0 {
            continue;
        }

        // Iterate into the first character of the word
        if let Some(first_character) = word.chars().next() {

            // Check it is a capital letter - pronoun
            if first_character.to_uppercase().next() == Some(first_character) {
                continue;
            }

            // Check if it is a symbol
            if first_character.is_ascii_punctuation() {
                continue;
            }

            // Run our word through the dictionary to find a match
            match check_dictionary(first_character, word) {
                Ok(found) => {
                    if found == true {
                        println!("Found the word");
                    }
                    else {

                        // Replace the word that couldn't be matched
                        println!("The word: {} does not exist in the dictionary", word);

                    }
                }
                Err(err) => {
                    println!("Error attempting to locate the word in the dictionary: {}", err);
                }
            }

        }
    }

    Ok(parsed_response)
}

fn check_dictionary(index: char, word: &str) -> Result<bool, Box<dyn std::error::Error>> {

    let mut dictionary_match = false;

    // Open the related dictionary
    let path = format!("dictionary/{}.txt", index);
    println!("Dictionary path: {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Iterate through to find our word
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                if line_content == *word {
                    dictionary_match = true;
                    break;
                }
            }
            Err(err) => {
                println!("Error matching: {} || Error: {}", word, err);
                break;
            }
        }
    }

    Ok(dictionary_match)
}