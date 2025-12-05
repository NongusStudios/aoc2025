use std::{fs::File, io::{self, BufRead, BufReader}, str::Chars};

#[derive(Debug, Clone, Copy)]
enum Rotation {
    L(i32),
    R(i32),
}

const START: i32 = 0;
const END: i32 = 99;

fn crack_password(sequence: &[Rotation]) -> i32 {
    let mut position = 50;
    let mut password = 0;

    for rotation in sequence {
        let mut pos = match rotation {
            Rotation::L(l) => position - l,
            Rotation::R(r) => position + r,
        };
        
        // pos = 50
        // L463
        // pos = -413
        // pos + 100 = -313
        // pos + 100 = -213
        // pos + 100 = -113
        // pos + 100 = -13
        // pos + 100 = 87
        // ------------------
        // pos = 50
        // R325
        // pos = 375
        // pos - 100 = 275
        // pos - 100 = 175
        // pos - 100 = 75
        while pos > END || pos < START {
            if pos > END {
                pos -= END+1;
            } else if pos < START {
                pos += END+1;
            }
        }

        position = dbg!(pos);

        if position == 0 { password += 1; }
    }

    password
}

fn scan_number(chs: &mut Chars<'_>) -> Option<i32> {
    let mut number = String::new();

    for ch in chs {
        if !ch.is_digit(10) { return None; }
        number.push(ch);
    }

    if let Ok(n) = number.parse::<i32>() {
        Some(n)
    } else { None }
}

fn line_count(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    
    Ok(BufReader::new(file).lines().count())
}

fn main() -> io::Result<()> {
    // Parse sequence from file
    let path = "sequence.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sequence: Vec<Rotation> = Vec::with_capacity(line_count(path)?);
    for line in reader.lines() {
        let line = line?;
        let mut chs = line.chars();
        
        if let Some(op) = chs.next() {
            match op {
                'R' => sequence.push( Rotation::R(scan_number(&mut chs).unwrap()) ),
                'L' => sequence.push( Rotation::L(scan_number(&mut chs).unwrap()) ),
                _   => continue,
            }
        }
    }

    // Crack password
    println!("Password = {}", crack_password(sequence.as_slice()));

    Ok(())
}
