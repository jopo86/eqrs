use std::io::{self, Write};

fn main() {

    println!("Enter \"<quit>\" to quit.\n");

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush output");

        let mut response = "".to_string();
        io::stdin().read_line(&mut response).expect("failed to read line");

        if response.trim().to_lowercase() == "<quit>" {
            break;
        } else {
            println!("{}\n", eqrs::eval(response.as_str(), None));
        }
    }
}
