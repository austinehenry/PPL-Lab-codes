fn extract_word(slice: &str) -> &str {
    // The function takes a string slice and extracts the first word (as an example)
    // It assumes that words are separated by spaces.
    let words: Vec<&str> = slice.split_whitespace().collect();
    words.get(0).copied().unwrap_or("") // Return the first word, or an empty string if no word exists
}

fn main() {
    // Accept input from the user
    let mut input = String::new();
    println!("Enter a sentence:");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the newline character
    let input = input.trim();

    // Use the extract_word function to extract a word (e.g., the first word)
    let word = extract_word(input);

    // Print the extracted word
    println!("Extracted word: {}", word);

    // Modify the original string by replacing the extracted word with "processed"
    let modified_input = input.replace(word, "processed");

    // Print the modified string
    println!("Modified sentence: {}", modified_input);

    // The original input remains unchanged, and the extracted word remains valid.
}
