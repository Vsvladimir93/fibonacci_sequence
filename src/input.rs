use std::io;

pub fn read_uint() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(n) => n,
        _ => {
            println!("Failed to read number! Enter number again: ");
            read_uint()
        }
    }
}