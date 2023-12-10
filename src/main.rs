use std::io;
use std::io::Write;
// Create a program that prompts for a quote and an author. Display the quotation and author as shown in the example output.

// verbs: prompts, display
// nouns: quote, author

// Input: quote and author
// Process: Build the output
// Output: the output

// Test
// Input: What is the quote?: These aren't the droids you're looking for., Who said it?: Obi-Wan Kenobi
// Output: Obi-Wan Kenobi says, "These aren't the droids you're looking for."



fn main() -> io::Result<()> {
    let mut quote = String::new();
    let mut author = String::new();
    print!("What is the quote?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut quote)?;
    print!("Who said it?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut author)?;
    
    println!("{}", author.trim().to_owned() + " says, " + "\"" + quote.trim() + "\"");
    Ok(())
}