use std::{fs::File, io::{self, BufRead, BufReader}, time::Instant};
use itertools::Itertools;

fn read_batteries(path: &str) -> io::Result<Vec<String>> {
    let reader = BufReader::new(File::open(path)?);
    
    let mut banks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        banks.push(line); 
    }

    Ok(banks)
}

// amount specifies the amount of batteries to turn on in each bank.
fn get_joltage(banks: &Vec<String>, amount: usize) -> u64 {
    let mut sum = 0;
    for bank in banks.iter() {
        let mut start = 0;
        let mut result = Vec::with_capacity(amount);
        let chars: Vec<char> = bank.chars().collect();

        for i in 0..amount {
            let end = chars.len() - (amount - i - 1);
            
            let mut max_digit = '0';
            let mut max_pos = start;

            for pos in start..end {
                if chars[pos] > max_digit {
                    max_digit = chars[pos];
                    max_pos = pos;
                }
            }

            result.push(max_digit);
            start = max_pos + 1;
        }

        sum += result.iter().collect::<String>().parse::<u64>().unwrap();
    }

    sum
}

fn main() {
    let banks = read_batteries("batteries.txt").unwrap();

    let start = Instant::now();
    println!("Found max joltage {}, in {:.3}s", get_joltage(&banks, 12), start.elapsed().as_secs_f64());
}
