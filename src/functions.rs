

    pub fn next() {
        println!("\n\n\t\t\t       Enter anything to continue.");
        let mut response = String::new();
        std::io::stdin().read_line(&mut response).expect("Failed to read line");
    }

    pub fn get_input<T: std::str::FromStr>(question: &str) -> T {
        loop {
            use std::io::Write;
            print!("\n{question}"); // makes it look nicer
            std::io::stdout().flush().unwrap(); //prints the buffer
    
            let mut response = String::new();
            std::io::stdin().read_line(&mut response).expect("Failed to read line");
            match response.trim().parse() {
                Ok(val) => return val,
                Err(_) => {println!("Error. Enter correct type.");continue;},
            }
        }
    }    