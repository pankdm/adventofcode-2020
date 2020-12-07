use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(filename: &str) -> Vec<String> {
    let full_name = format!("{}", filename);
    let msg = format!("File {} not found", full_name);
    let file = File::open(full_name).expect(msg.as_str());
    let reader = BufReader::new(file);
    let mut res = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        res.push(line.to_string());
    }
    return res;
}

pub fn to_v_char(line: &str) -> Vec<char> {
    line.chars().collect()
}

pub fn to_vv_char(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

pub fn parse_i64(s: &str) -> i64 {
    match s.parse::<i64>() {
        Err(e) => {
            assert!(false, "Error parsing '{}': {}", &s, e);
            unreachable!();
        }
        Ok(value) => {
            return value;
        }
    }
}

pub fn split_string(s: &str, pattern: &str) -> Vec<String> {
    let mut res = Vec::new();
    for part in s.split(pattern) {
        res.push(part.to_string());
    }
    return res;
}

pub fn to_lines(s: &str) -> Vec<String> {
    split_string(&s.trim().to_string(), "\n")
}
