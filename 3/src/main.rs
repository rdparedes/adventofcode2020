use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

struct Map {
    data: Vec<Vec<char>>,
    height: u32,
    width: u32,
    x: u32,
    y: u32,
}

impl Map {
    fn new() -> Map {
        Map {
            data: Vec::new(),
            x: 0,
            y: 0,
            height: 0,
            width: 0,
        }
    }

    fn add_row(&mut self, line: String) {
        let row: Vec<char> = line.chars().collect();
        self.width = row.len() as u32;
        self.height += 1;
        self.data.push(row);
    }

    fn go_right(&mut self, times: u32) {
        self.x += times;
        if self.x >= (self.width) {
            self.x -= self.width;
        }
    }

    fn go_down(&mut self, times: u32) {
        self.y += times;
        if self.y >= (self.height - 1) {
            self.y = self.height - 1;
        }
    }

    fn is_at_bottom(&self) -> bool {
        return self.y >= (self.height - 1);
    }

    fn reset_location(&mut self) {
        self.y = 0;
        self.x = 0;
    }

    fn get_current_location(&self) -> char {
        let col = self.x as usize;
        let row = self.y as usize;
        // println!("Current coords: {}, {}\n.Current char: {}", self.x, self.y, self.data[row][col]);
        return self.data[row][col];
    }
}

fn read_file(filename: &str) -> Result<Map> {
    let path = Path::new(&filename);
    let file = File::open(&path)?;
    let br = BufReader::new(file);

    let mut m = Map::new();
    for line in br.lines().filter_map(|result| result.ok()) {
        m.add_row(line);
    }
    return Ok(m);
}

fn main() {
    let mut m = read_file("src/input.txt").unwrap();

    let test_data: [(u32, u32); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut results: Vec<u64> = Vec::new();

    for (right, down) in test_data.iter() {
        let mut tree_counter: u64 = 0;
        m.reset_location();
        while !m.is_at_bottom() {
            m.go_right(*right);
            m.go_down(*down);
            if m.get_current_location() == '#' {
                tree_counter += 1;
            }
        }
        results.push(tree_counter);
    }


    println!("Result: {}", results.iter().product::<u64>());
}
