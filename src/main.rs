mod input;

use std::vec;

fn main() {
    println!("Please enter size of fibonacci sequence: ");

    let sequence_size = input::read_uint();

    let sequence = generate_fibonacci_sequence(sequence_size);

    println!("{:?}", sequence);
}

fn generate_fibonacci_sequence(size: usize) -> Vec<u64> {
    let mut sequence = vec![1; size];

    if size > 2 {
        for n in 2..size {
            sequence[n] = sequence[n - 2] + sequence[n - 1];
        }
    }

    return sequence;
}
