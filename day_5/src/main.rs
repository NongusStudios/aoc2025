
struct DB {
    fresh_ranges: Vec<(usize, usize)>,
    ingredient_ids: Vec<usize>,
}

impl DB {
    fn new(path: &str) -> io::Result<Self> {

        Self {
            fresh_ranges:   Vec::new(),
            ingredient_ids: Vec::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
