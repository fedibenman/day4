use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Replace "your_file.txt" with the path to your text file
    let file = File::open("file.txt")?;
    let reader = io::BufReader::new(file);

    // Vector to store the results for each line
    let mut sum = 0;

 
    // Iterate over lines in the file
    for line in reader.lines() {
        // Handle each line as needed
        match line {
            Ok(text) => {
                // Find the index of ":" and take the substring after it
                let after_colon: String = text.split(':').skip(1).collect();
                // Split the remaining string by ","
                let split_strings: Vec<&str> = after_colon.split('|').flat_map(|s| s.split(',')).collect();
                let winning_string: &str = split_strings[0];
                let your_string: &str = split_strings[1];
                let winning_numbers: Vec<&str> = winning_string.split_whitespace().collect();
                let your_numbers: Vec<&str> = your_string.split_whitespace().collect();
                
                let common_numbers: Vec<&&str> = winning_numbers.iter().filter(|&x| your_numbers.contains(x)).collect();
                let common_count = common_numbers.len();
                sum += if common_count>0 {2u32.pow(common_count as u32 -1) } else {0};
            }
            Err(err) => eprintln!("Error reading line: {}", err),
        }
 


}
println!("sum = {} ",sum) ;
                
    Ok(())
}