// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};

use serde_scan;

use crate::utils::*;

fn check_pass1(fields: &HashMap<String, String>) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for r in required.iter() {
        if !fields.contains_key(&r.to_string()) {
            return false;
        }
    }
    true
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)

    let mut index = 0;
    let mut fields = HashMap::new();
    loop {
        let line = if index < lines.len() {
            lines[index].clone()
        } else {
            "".to_string()
        };
        if line.is_empty() {
            if check_pass1(&fields) {
                res += 1;
            }
            fields = HashMap::new();
        } else {
            let parts = split_string(&line, " ");
            // println!("parts = {:?}", parts);
            for part in parts.iter() {
                let kv = split_string(part, ":");
                fields.insert(kv[0].clone(), kv[1].clone());
            }
        }
        if index >= lines.len() {
            break;
        }
        index += 1;
    }
    res
}

fn is_all_digits(s: &String) -> bool {
    s.chars().all(|c| c.is_numeric())
}

fn check_digits(s: &String, min: i64, max: i64) -> bool {
    if !is_all_digits(s) {
        return false;
    }
    let value = parse_i64(s);
    return min <= value && value <= max;
}

fn check_hgt(s: &String) -> bool {
    if s.ends_with("cm") {
        let prefix = s[..s.len() - 2].to_string();
        return check_digits(&prefix, 150, 193);
    } else if s.ends_with("in") {
        let prefix = s[..s.len() - 2].to_string();
        return check_digits(&prefix, 59, 76);
    } else {
        return false;
    }
}

fn check_hcl(s: &String) -> bool {
    if s.len() != 7 {
        return false;
    }
    if s.as_bytes()[0] != b'#' {
        return false;
    }
    for i in 1..s.len() {
        let ch = s.as_bytes()[i] as char;
        if !(('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'f')) {
            return false;
        }
    }
    return true;
}

fn check_ecl(s: &String) -> bool {
    let opts = split_string(&"amb blu brn gry grn hzl oth".to_string(), " ");
    opts.contains(s)
}

fn check_pass2(fields: &HashMap<String, String>) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for r in required.iter() {
        if !fields.contains_key(&r.to_string()) {
            return false;
        }
    }

    for (key, v) in fields.iter() {
        let ok = match key.as_str() {
            "byr" => check_digits(v, 1920, 2002),
            "iyr" => check_digits(v, 2010, 2020),
            "eyr" => check_digits(v, 2020, 2030),
            "hgt" => check_hgt(v),
            "hcl" => check_hcl(v),
            "ecl" => check_ecl(v),
            "pid" => v.len() == 9 && is_all_digits(v),
            _ => true,
        };
        if !ok {
            return false;
        }
    }
    return true;
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)

    let mut index = 0;
    let mut fields = HashMap::new();
    loop {
        let line = if index < lines.len() {
            lines[index].clone()
        } else {
            "".to_string()
        };
        if line.is_empty() {
            if check_pass2(&fields) {
                res += 1;
            }
            fields = HashMap::new();
        } else {
            let parts = split_string(&line, " ");
            // println!("parts = {:?}", parts);
            for part in parts.iter() {
                let kv = split_string(part, ":");
                fields.insert(kv[0].clone(), kv[1].clone());
            }
        }
        if index >= lines.len() {
            break;
        }
        index += 1;
    }
    res
}

pub fn read_main_input() -> Vec<String> {
    read_input("input/day04/in.txt")
    // unreachable!()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1() {
        let lines = read_main_input();
        assert_eq!(part1(&lines), 260);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 153);
    }
}
