

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
                Err(_) => {println!("\n\t\t\tError. Enter correct type.");continue;},
            }
        }
    }    

    pub fn menu(options: u8) -> u8 {
        loop {
            let num:u8 = get_input("\n\tEnter selection: ");
            if num >= 1 && num <= options {
                return num;
            }
            else {
                println!("\n\n\t\tPlease enter a valid selection.");
                continue;
            }
        }
    }