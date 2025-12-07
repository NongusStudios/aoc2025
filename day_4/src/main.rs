use std::{fs::File, io::{self, Read}};

fn read_map(path: &str) -> io::Result<Vec<Vec<char>>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let rows = buf.trim_end_matches('\n').split('\n');
    let mut result: Vec<Vec<char>> = Vec::new();

    for row in rows.into_iter() {
        let mut result_row: Vec<char> = Vec::new();

        for block in row.chars() {
            if block.is_whitespace() { continue; }
            match block {
                '.' | 
                '@' => result_row.push(block),
                _ => continue,
            }
        }

        result.push(result_row);
    }

    Ok(result)
}

fn print_banner() {
println!(
"|----------------------------------------------|
|***********************************************|
|*       Advent of code 2025 - Day 4           *|      
|***********************************************|
|-----------------------------------------------|

Each turn all paper rolls with less than 4 adjacent neigbours is removed,
until no more can be reached.
");
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn accessible_rolls(map: &mut Vec<Vec<char>>) -> io::Result<u64> {
    let mut sum = 0;

    let mut preview = map.clone();
    
    print_banner();
    print_map(&preview);
    println!("\nPress Enter to continue.");
    io::stdin().read_line(&mut String::new())?;
    print!("\x1B[2J\x1B[1;1H"); /* Clear Terminal */
    
    loop {
        let mut remove_queue: Vec<(usize, usize)> = Vec::new(); 
        for (y, row) in map.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if *item == '.' { continue; }

                // Start checking adjacent items in the top left cell
                let (startx, starty) = (((x as i64) - 1).max(0), ((y as i64 - 1)).max(0));
                let (endx, endy) = ((x + 1).min(map[y].len()-1) as i64, (y + 1).min(map.len()-1) as i64);
               
                let mut adjacent_rolls = 0;
                for adjy in (starty..=endy).into_iter() {
                    for adjx in (startx..=endx).into_iter() {
                        if adjy == y as i64 && adjx == x as i64 { continue; } // Skip self
                        if map[adjy as usize][adjx as usize] == '@' { 
                            adjacent_rolls += 1;
                        }
                    }
                }

                if adjacent_rolls < 4 {
                    preview[y][x] = 'X'; 
                    remove_queue.push((x, y));
                    sum += 1;
                }
            }
        }
        
        for (x, y) in remove_queue.iter() {
            map[*y][*x] = '.';
        }
        
        if remove_queue.is_empty() { break; }
        print_map(&preview);
        println!("Removed {} rolls.", remove_queue.len());
        remove_queue.clear();

        println!("\nPress Enter to continue.");
        io::stdin().read_line(&mut String::new())?;
        print!("\x1B[2J\x1B[1;1H"); /* Clear Terminal */
    }

    print_map(&preview);

    Ok(sum)
}

fn main() {
    let mut map = read_map("map.txt").unwrap();
    println!("Total rolls removed {}", accessible_rolls(&mut map).unwrap()) 
}
