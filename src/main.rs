use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Random Password Generator!");

    // Ask the user for the length of the password
    let length: usize = match get_input("Enter the length of the password: ").trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default length of 12.");
            12 // Default password length
        }
    };

    // Generate a random password
    let password = generate_password(length);
    println!("Generated Password: {}", password);
}

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            ~!@#$%^&*()-_+=";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
