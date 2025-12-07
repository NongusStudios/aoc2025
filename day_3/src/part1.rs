use std::{fs::File, io::{self, BufRead, BufReader}};


fn read_batteries(path: &str) -> io::Result<Vec<String>> {
    let reader = BufReader::new(File::open(path)?);
    
    let mut banks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        banks.push(line); 
    }

    Ok(banks)
}

fn get_joltage(banks: &Vec<String>) -> u64 {
    let mut sum = 0;
    for (_, bank) in banks.iter().enumerate() {
        let row = bank.clone();
        let mut largest = 0;
        for (i, battery) in bank.chars().enumerate() {
             for other in row.chars().skip(i+1) {
                let joltage = [battery, other]
                    .iter().collect::<String>()
                    .parse::<u64>().unwrap();
                if joltage > largest {
                    largest = joltage;
                }
            }
        }
        sum += largest;
    }

    sum
}

fn main() {
    let banks = read_batteries("batteries.txt").unwrap();
    println!("Max joltage: {}", get_joltage(&banks));
}
