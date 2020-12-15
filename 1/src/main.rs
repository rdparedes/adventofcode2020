use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::Path;

fn read_file(filename: &str) -> Result<Vec<i64>> {
    let path = Path::new(&filename);

    let file = File::open(&path)?;
    let br = BufReader::new(file);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let data: Vec<i64> = read_file("src/input.txt")
        .unwrap()
        .drain(..)
        .filter(|v| *v <= 1999)
        .collect();

    // Brute force sollution
    'outer: for i in 0..data.len() {
        for j in 0..data.len() {
            let i_val = data[i];
            let j_val = data[j];
            let partial_sum = i_val + j_val;
            if i == j || partial_sum > 2020 {
                continue;
            }
            for k in 0..data.len() {
                if k == j || k == i {
                    continue;
                }
                let k_val = data[k];
                let k_partial_sum = partial_sum +  k_val;
                if k_partial_sum == 2020 {
                    println!("Found one!: {}", i_val * j_val * k_val);
                    break 'outer;
                }
            }
        }
    }
}
