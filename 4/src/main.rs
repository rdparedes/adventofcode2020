use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn new(raw_data: HashMap<String, String>) -> Option<Passport> {
        let byr_ = raw_data.get("byr");
        let iyr_ = raw_data.get("iyr");
        let eyr_ = raw_data.get("eyr");
        let hgt_ = raw_data.get("hgt");
        let hcl_ = raw_data.get("hcl");
        let ecl_ = raw_data.get("ecl");
        let pid_ = raw_data.get("pid");
        if byr_.is_none()
            || iyr_.is_none()
            || eyr_.is_none()
            || hgt_.is_none()
            || hcl_.is_none()
            || ecl_.is_none()
            || pid_.is_none()
        {
            return None;
        }
        Some(Passport {
            byr: byr_.unwrap().parse::<u16>().unwrap(),
            iyr: iyr_.unwrap().parse::<u16>().unwrap(),
            eyr: eyr_.unwrap().parse::<u16>().unwrap(),
            hgt: hgt_.unwrap().to_string(),
            hcl: hcl_.unwrap().to_string(),
            ecl: ecl_.unwrap().to_string(),
            pid: pid_.unwrap().to_string(),
        })
    }

    fn height_is_valid(&self) -> bool {
        let height_regex = Regex::new(r"^([0-9]{2,3})(cm|in)$").expect("invalid height Regex");
        if height_regex.is_match(&self.hgt) {
            let splits = height_regex.captures(&self.hgt).unwrap();
            let quantity: u16 = splits
                .get(1)
                .map_or("", |m| m.as_str())
                .parse()
                .unwrap_or(0);
            let measure = splits.get(2).map_or("", |m| m.as_str());
            if measure == "cm" {
                return quantity >= 150 && quantity <= 193;
            } else if measure == "in" {
                return quantity >= 59 && quantity <= 76;
            }
            return false;
        }
        return false;
    }

    fn hair_color_is_valid(&self) -> bool {
        let color_regex = Regex::new(r"^\#([A-Fa-f0-9]{6})$").expect("invalid hair color Regex");
        if color_regex.is_match(&self.hcl) {
            return true;
        }
        return false;
    }

    fn eye_color_is_valid(&self) -> bool {
        let eye_color_regex =
            Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").expect("invalid eye color Regex");
        if eye_color_regex.is_match(&self.ecl) {
            return true;
        }
        return false;
    }

    fn passport_id_is_valid(&self) -> bool {
        let passport_regex = Regex::new(r"^[0-9]{9}$").expect("invalid passport Regex");
        if passport_regex.is_match(&self.pid) {
            return true;
        }
        return false;
    }

    fn is_valid(&self) -> bool {
        if (self.byr >= 1920 && self.byr <= 2002)
            && (self.iyr >= 2010 && self.iyr <= 2020)
            && (self.eyr >= 2020 && self.eyr <= 2030)
            && self.height_is_valid()
            && self.hair_color_is_valid()
            && self.eye_color_is_valid()
            && self.passport_id_is_valid()
        {
            return true;
        }
        return false;
    }
}

fn parse_file(filename: &str) -> Result<Vec<Passport>> {
    let path = Path::new(&filename);
    let file = File::open(&path)?;
    let br = BufReader::new(file);

    let mut response: Vec<Passport> = Vec::new();
    let mut current_element = HashMap::new();

    let mut lines_iter = br.lines().map(|l| l.unwrap());

    loop {
        let line = lines_iter.next();
        let l = line.clone().unwrap_or("".to_string());
        if l.is_empty() {
            match Passport::new(current_element) {
                Some(p) => response.push(p),
                None => (),
            }
            current_element = HashMap::new();

            let eof = !line.is_some();
            if eof {
                break;
            }
        } else {
            for word in l.split_whitespace() {
                let parts: Vec<String> = word.split(':').map(|w| w.to_string()).collect();
                current_element.insert(
                    parts.first().unwrap().to_string(),
                    parts.last().unwrap().to_string(),
                );
            }
        }
    }

    return Ok(response);
}

fn main() {
    let mut data: Vec<Passport> = parse_file("src/input.txt").unwrap();

    println!("Total: {}", data.len());

    data = data.drain(..).filter(|p| p.is_valid()).collect();

    println!("Valid passports: {}", data.len());
}
