#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{self, BufRead, BufReader, Write},
    };

    use regex::Regex;

    #[test]
    fn security_check() {
        let file_path = "input.txt";

        // Open the input file and print to standard error
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error opening the file: {}", error);
                return;
            }
        };
        // Create a BufReader to efficiently read the file line by line
        let reader = BufReader::new(file);

        // Define a regular expression to match potential passwords or API keys
        let password_regex = Regex::new(r"(?i)(password|api[_\s]?key)[:=]\s*(\w+)").unwrap();

        // Perform the security check by scanning each line of the file
        for (line_number, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    eprintln!("Error reading line {}: {}", line_number + 1, error);
                    continue;
                }
            };

            // Search for matches in the current line using the password_regex
            if password_regex.is_match(&line) {
                println!(
                    "Potential security issue found in line {}: {}",
                    line_number + 1,
                    line
                );
            }
        }
    }

    #[test]
    fn io() -> io::Result<()> {
        let file = File::open("input.txt")?;
        let reader = BufReader::new(file);
        // Open the output file for writing
        let mut output_file = File::create("output.txt")?;

        // Read lines from the input file and process them
        for line in reader.lines() {
            let input_line = line?;
            let number: i32 = input_line.trim().parse().expect("Invalid number in file");

            // Perform a simple operation (multiply by 2 in this case)
            let result = number * 2;

            // Print the result to stdout
            println!("Input: {}, Output: {}", number, result);

            // Write the result to the output file
            writeln!(output_file, "{}", result)?;
        }

        Ok(())
    }
}
