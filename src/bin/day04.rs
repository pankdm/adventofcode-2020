// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};

use serde_scan;

extern crate aoc;
use aoc::*;

pub fn part1(lines_: &Vec<String>) -> i64 {
    let mut res = 0;

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
    let mut lines = lines_.clone();
    lines.push("".to_string());

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut inside = false;
    let mut fields = HashMap::new();
    for line in lines.iter() {
        let parts = split_string(line, " ");
        if line.len() == 0 || parts.is_empty() {
            let mut ok = true;
            // println!("  checking {:?}", fields);
            for f in required.iter() {
                if !fields.contains_key(&f.to_string()) {
                    ok = false;
                }
            }
            if ok {
                res += 1;
            }
            fields = HashMap::new();
            continue;
        }

        // println!("parts = {:?}", parts);
        for part in parts.iter() {
            let kv = split_string(part, ":");
            fields.insert(kv[0].clone(), kv[1].clone());
        }
    }
    res
}


fn is_all_digits(s: &String) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
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
        let prefix = s.as_str()[0..s.len() - 2].to_string();
        return check_digits(&prefix, 150, 193);
    } else if s.ends_with("in") {
        let prefix = s.as_str()[0..s.len() - 2].to_string();
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
        if ! (('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'f')) {
            return false;
        }
    }
    return true;
}

fn check_ecl(s: &String) -> bool {
    let opts = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for opt in opts.iter() {
        if s == opt {
            return true;
        }
    }
    return false;
}

fn check_pass(fields: &HashMap<String, String>) -> bool {
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
    for f in required.iter() {
        let ff = f.to_string();
        if !fields.contains_key(&ff) {
            return false;
        }
        let v = &fields[&ff];
        let ok = match f {
            &"byr" => check_digits(v, 1920, 2002),
            &"iyr" => check_digits(v, 2010, 2020),
            &"eyr" => check_digits(v, 2020, 2030),
            &"hgt" => check_hgt(v),
            &"hcl" => check_hcl(v),
            &"ecl" => check_ecl(v),
            &"pid" => v.len() == 9 && is_all_digits(v),
            _ => unreachable!("field: {}", f),
        };
        if !ok {
            return false;
        }
    }
    return true;
}

pub fn part2(lines_: &Vec<String>) -> i64 {
    let mut res = 0;

    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
    let mut lines = lines_.clone();
    lines.push("".to_string());


    let mut inside = false;
    let mut fields = HashMap::new();
    for line in lines.iter() {
        let parts = split_string(line, " ");
        if line.len() == 0 || parts.is_empty() {
            // println!("  checking {:?}", fields);
            let mut ok = check_pass(&fields);
            if ok {
                res += 1;
            }
            fields = HashMap::new();
            continue;
        }

        // println!("parts = {:?}", parts);
        for part in parts.iter() {
            let kv = split_string(part, ":");
            fields.insert(kv[0].clone(), kv[1].clone());
        }
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
        assert_eq!(part1(&lines), -1);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), -1);
    }
}

fn main() {
    let lines = read_main_input();
    // let lines = read_input("input/day04/t1.txt");


    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
