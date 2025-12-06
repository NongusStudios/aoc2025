
/*
* Invalid IDs
* - Has a sequence of digit repeated twice (55, 6464, 123123)
* - and/or has a lead zero 
* */

use std::{fs::File, io::{self, Read}};

fn is_repeated_char(s: &str) -> bool {
    match s.chars().next() {
        Some(first) => s.chars().all(|c| c == first),
        None => true,
    }
}

fn has_repeated_sequence(id: u64) -> bool {
    let mut sid = id.to_string(); 
    
    if sid.len() % 2 != 0 { return false; }
    if is_repeated_char(&sid) { return true; }

    let second = sid.split_off(sid.len()/2);
    sid == second
}

fn read_to_end(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;

    let mut src: Vec<u8> = Vec::new();
    file.read_to_end(&mut src)?;

    Ok(src)
}

fn parse_invalid_ids() -> Result<Vec<u64>, String> {
    let src: Vec<u8>;
    match read_to_end("ranges.txt") {
        Ok(s) => src = s,
        Err(_) => { return Err("failed to read file ranges.txt".to_string()); }
    }
    
    if let Ok(s) = String::from_utf8(src) {
        let s = s.trim_end_matches('\n');
        let mut ids: Vec<u64> = Vec::new();
        
        let csv = s.split(',');
        for range in csv { 
            let mut bounds = range.split('-');
            let id_range: Vec<u64> = (
                bounds.next().unwrap().parse::<u64>().unwrap()
                ..
                bounds.next().unwrap().parse::<u64>().unwrap()+1
            ).collect();

            for (_, id) in id_range.iter().enumerate() {
                if has_repeated_sequence(*id) { ids.push(*id); }
            }
        }

        Ok(ids)
    } else {
        Err("failed to convert file data to UTF-8 string.".to_string())
    }
}

fn main() {
    let ids = dbg!(parse_invalid_ids().unwrap());
    
    let mut sum: u64 = 0;
    ids.iter().for_each(|id| sum += id);

    println!("Sum of all invalid ids: {}", sum);
}
