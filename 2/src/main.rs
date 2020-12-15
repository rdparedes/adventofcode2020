use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

struct Policy {
    letter: char,
    min: u16,
    max: u16,
}

impl Policy {
    fn new(raw_data: &str) -> Policy {
        let letter = raw_data.chars().last().unwrap();
        let min_max = raw_data.split_whitespace().next().unwrap_or("");
        let mut min_max_iter = min_max.split('-');
        let min: u16 = (min_max_iter.next().unwrap()).parse().unwrap();
        let max: u16 = (min_max_iter.next().unwrap()).parse().unwrap();
        Policy {
            letter: letter,
            min: min,
            max: max,
        }
    }
}

struct Password {
    value: String,
    policy: Policy,
}

impl Password {
    fn new(raw_data: &str) -> Password {
        let item: Vec<&str> = raw_data.split(':').map(|s| s.trim()).collect();
        let value = String::from(item[1]);
        let policy = Policy::new(&item[0]);
        Password {
            value: value,
            policy: policy,
        }
    }

    fn isvalid(&self) -> bool {
        let chars: Vec<char> = self.value.chars().collect();
        let mut is_valid = false;

        // check first incidence
        let mut letter_index = usize::from(self.policy.min - 1);
        let first_letter = chars.get(letter_index).unwrap();
        if self.policy.letter == *first_letter {
            is_valid = !is_valid;
        }
        // check second incidence
        letter_index = usize::from(self.policy.max - 1);
        let second_letter = chars.get(letter_index).unwrap();
        if self.policy.letter == *second_letter {
            is_valid = !is_valid;
        }
        return is_valid;
    }
}

fn read_file(filename: &str) -> Result<Vec<Password>> {
    let path = Path::new(&filename);
    let file = File::open(&path)?;
    let br = BufReader::new(file);

    br.lines()
        .map(|line| line.and_then(|v| Ok(Password::new(&v))))
        .collect()
}

fn main() {
    let mut data: Vec<Password> = read_file("src/input.txt").unwrap();

    data = data.drain(..).filter(|v| v.isvalid()).collect();

    println!("{}", data.len())
}
