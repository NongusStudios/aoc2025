use std::{collections::HashSet, fs::File, io::{self, BufRead, BufReader}};

#[derive(Debug)]
struct DB {
    fresh_ranges: Vec<(usize, usize)>,
    ingredient_ids: Vec<usize>,
}

impl DB {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        
        let mut fresh_ranges = Vec::new();
        let mut ingredient_ids = Vec::new();

        for line in BufReader::new(file).lines() {
            let     line = line?;
            let mut split = line.split('-');
            if split.clone().count() > 1 {
                let start = split.next().unwrap();
                let end   = split.next().unwrap();
                fresh_ranges.push((start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap()));
            } else {
                ingredient_ids.push(
                    line.parse::<usize>().unwrap()
                );
            }
        }

        fresh_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

        Ok(Self {
            fresh_ranges,
            ingredient_ids,
        })
    }

    fn fresh_id_count(&self) -> usize {
        let mut sum = 0;
        for id in self.ingredient_ids.iter() {
            for (start, end) in self.fresh_ranges.iter() {
                if id >= start && id <= end {
                    sum += 1;
                    break;
                }
            }
        }

        sum
    }

    fn number_of_valid_fresh_ids(&self) -> usize {
        let mut id_count = 0;
        let mut processed_ranges: Vec<(usize, usize)> = Vec::new();
        for (start, end) in self.fresh_ranges.iter() {
            let mut should_push = true;
            
            for (pstart, pend) in processed_ranges.iter_mut() {
                if start <= pend && end >= pstart {
                    *pstart = *start.min(pstart);
                    *pend   = *end.max(pend);
                    should_push = false;
                }
            }

            if should_push { processed_ranges.push((*start, *end)); }
        }

        for (start, end) in processed_ranges.iter() {
            id_count += (end - start) + 1;
        } 

        id_count
    }

    //fn number_of_valid_fresh_ids(&self) -> usize {
    //    let mut id_count = 0;
    //    for (i, (start, end)) in self.fresh_ranges.iter().enumerate() {
    //        let mut n = (end - start) + 1;
    //        println!("n = {}", n);
    //        for (other_start, other_end) in self.fresh_ranges.iter().skip(i+1) {
    //            if start <= other_end && end >= other_start {
    //                // Edge case if the other range fully encompasses the current range
    //                if start >= other_start && end <= other_end {
    //                    n = 0;
    //                    break;
    //                // Edge case if next range is encompassed by the current range
    //                } else if other_start >= start && other_end <= end {
    //                    n -= (other_end - other_start) + 1;
    //                    continue;
    //                }
    //                let sub = (other_end - start).min(end - other_start) + 1;
    //                if sub > n {
    //                    panic!("sub: {} > n: {}\ncurrent: {}, {}\n  other: {}, {}", sub, n, start, end, other_start, other_end);
    //                }
    //                n -= sub;
    //                println!("current: {}, {}\n  other: {}, {}\nn -= {} = {}", start, end, other_start, other_end, sub, n);
    //            }
    //        }
    //        println!("");
    //        id_count += n;
    //    }

    //    id_count
    //}
}

fn main() {
    let db = DB::new("db.txt").unwrap();
    println!("Number of fresh ingredients: {}", db.fresh_id_count());
    println!("Number of fresh ingredient ids: {}", db.number_of_valid_fresh_ids());
}
