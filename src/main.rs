use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename which will be the second element of the args
    let filename = args().nth(1).ok_or("Expected filename")?;
    // Open the file
    let file = File::open(filename)?;
    // Fail early if the file is empty
    if file.metadata()?.len() == 0 {
        return Err("File is empty".into());
    }
    // Wrap the file in a BufReader
    let file = BufReader::new(file);
    // A list of the characters we care about
    let bf_chars = ['+', '-', '<', '>', '.', ',', '[', ']'];

    let bf = file
        .lines()
        .map(|l| {
            // Map Ok value of Result
            l.map(|mut s| {
                // Remove any non-bf chars
                s.retain(|c| bf_chars.contains(&c));
                s
            })
        })
        .collect::<Result<Vec<String>, std::io::Error>>()?; // Collect as Result and check the error

    // I was using the below lines at one stage to produce and iterator for a for loop to print
    // each line of the bf program
    // I prefer the new solution for future proofing and also the fact that there is no for loop
    // needed
    // (These lines would've come after the collect() in case it isn't clear)
    //.iter() // Create an iterator which filters blank lines
    //.filter(|s| !s.is_empty())

    // Get the resulting bf program as a vector of chars (in prep for future)
    let bf: Vec<char> = bf.iter().flat_map(|s| s.chars()).collect();
    // Is this potentially a bad idea if the resulting bf program is massive? Presumably not really
    // any more troublesome than having the above Vec<char>
    println!("{}", bf.iter().collect::<String>());

    /* This is the first solution I came up with before messing with map()/collect()/filter()
     * These lines would replace the above for loop
    // For each line
    for line in file.lines() {
        let mut line = line?;
        // Keep only the characters in the list
        line.retain(|c| bf_chars.contains(&c));
        // Drop empty lines
        if !line.is_empty() {
            print!("{}", line);
        }
    }
    */

    Ok(())
}
