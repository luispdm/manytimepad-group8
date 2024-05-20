use std::io::{BufRead, BufReader, Result};
use std::fs::File;

extern crate hex;

fn xor_arrays(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let min_length = a.len().min(b.len());

    for i in 0..min_length {
        result.push(a[i] ^ b[i]);
    }

    // If arrays have different lengths, process remaining elements
    if a.len() > b.len() {
        for &byte in a.iter().skip(min_length) {
            result.push(byte);
        }
    } else if b.len() > a.len() {
        for &byte in b.iter().skip(min_length) {
            result.push(byte);
        }
    }

    result
}

fn check_pairs_for_string(file_path: &str, target_string: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    // Read lines from the file and store them in a vector
    for line in reader.lines() {
        lines.push(line?);
    }

    // Check pair-by-pair if any two lines contain the target string
    for (i, line1) in lines.iter().enumerate() {
        for (j, line2) in lines.iter().enumerate() {
            let byte_array_one = hex::decode(line1).unwrap();
            let byte_array_two = hex::decode(line2).unwrap();
            let hex_xor = hex::encode(xor_arrays(&byte_array_one, &byte_array_two));
            
            if i != j && (hex_xor.contains(target_string)) {
                println!("Lines {} and {} contain the target string.", i + 1, j + 1);
            }
        }
    }

    Ok(())
}

fn main() {
    println!("{:?}", check_pairs_for_string("ciphertexts.txt", "746865"));
}
