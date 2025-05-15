use std::io::{self, Write, stdin};

pub fn init() {
    loop {
        print!("Ferrish > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // Panic | STD | Error Handling
        if stdin().read_line(&mut input).is_err() {
            println!("Error Reading Input");
            continue;
        };

        // White Space Cleanup
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // tokenized input
        let input_tokenized: Vec<&str> = input.split_whitespace().collect();

        // exit command
        if input.to_lowercase() == "exit" {
            break;
        }
    }
}
