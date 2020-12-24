// Disable some unhelpful warnings
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// Some basic includes to alwawys include
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

use serde_scan;

use crate::utils::*;

#[derive(PartialEq, Clone, Debug)]
enum Rule {
    Symbol(char),
    Or(Vec<Vec<i64>>),
}

fn matches_vec(
    next_rules: &Vec<i64>,
    rules: &HashMap<i64, Rule>,
    s: &Vec<char>,
    index: usize,
) -> (bool, usize) {
    let mut current = index;
    for next in next_rules.iter() {
        let (ok, end) = matches_impl(*next, rules, s, current);
        if !ok {
            return (false, index);
        }
        current = end;
    }
    return (true, current);
}

fn matches_impl(id: i64, rules: &HashMap<i64, Rule>, s: &Vec<char>, index: usize) -> (bool, usize) {
    let rule = &rules[&id];
    match rule {
        Rule::Symbol(c) => {
            if index >= s.len() {
                return (false, index);
            }
            if s[index] != *c {
                return (false, index);
            }
            return (true, index + 1);
        }
        Rule::Or(ors) => {
            for a in ors.iter() {
                let (ok, end) = matches_vec(a, rules, s, index);
                if ok {
                    return (ok, end);
                }
            }
            return (false, index);
        }
    }
    unreachable!()
}

fn matches(rules: &HashMap<i64, Rule>, line: &String) -> bool {
    let s = to_v_char(line);
    let (ok, end) = matches_impl(0, rules, &s, 0);
    ok && end == line.len()
}

fn matches_0(rules: &mut HashMap<i64, Rule>, line: &String) -> bool {
    let n = 10;

    for n8 in 1..n {
        for n11 in 1..n {
            let mut ors8 = Vec::new();
            {
                let mut value = Vec::new();
                for j in 0..n8 {
                    value.push(42);
                }
                ors8.push(value);
            }
            let rule8 = Rule::Or(ors8);

            let mut ors11 = Vec::new();
            {
                let mut value = Vec::new();
                for j in 0..n11 {
                    value.push(42);
                }
                for j in 0..n11 {
                    value.push(31);
                }
                ors11.push(value);
            }
            let rule11 = Rule::Or(ors11);

            rules.insert(8, rule8);
            rules.insert(11, rule11);
            if matches(rules, line) {
                return true;
            }
        }
    }
    return false;
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut rules = HashMap::new();

    let mut index = 0;
    loop {
        let line = &lines[index];
        if line.is_empty() {
            break;
        }
        let parts = split_string(line.trim(), ": ");
        let id = parse_i64(&parts[0]);

        if parts[1].starts_with('"') {
            rules.insert(id, Rule::Symbol(parts[1].chars().nth(1).unwrap()));
        } else {
            let tokens = split_string(&parts[1], " | ");
            let mut ors = Vec::new();
            for token in tokens.iter() {
                let r0 = parse_ints(token, " ");
                ors.push(r0);
            }
            let rule = Rule::Or(ors);
            rules.insert(id, rule);
        }

        if parts.is_empty() {
            break;
        }

        index += 1;
    }

    // dbg!(&rules);

    index += 1;
    loop {
        if index >= lines.len() {
            break;
        }
        let line = &lines[index];

        if matches(&rules, line) {
            // println!("matches: {}", line);
            res += 1;
        }
        index += 1;
    }

    // TODO: code here
    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;

    let mut rules = HashMap::new();

    let mut index = 0;
    loop {
        let line = &lines[index];
        if line.is_empty() {
            break;
        }
        let parts = split_string(line.trim(), ": ");
        let id = parse_i64(&parts[0]);

        if parts[1].starts_with('"') {
            rules.insert(id, Rule::Symbol(parts[1].chars().nth(1).unwrap()));
        } else {
            let tokens = split_string(&parts[1], " | ");
            let mut ors = Vec::new();
            for token in tokens.iter() {
                let r0 = parse_ints(token, " ");
                ors.push(r0);
            }
            let rule = Rule::Or(ors);
            rules.insert(id, rule);
        }

        if parts.is_empty() {
            break;
        }

        index += 1;
    }

    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    // rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    // rules.insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    // rule[11] = Or([42, 31], [42, 11, 31])
    // rule[8] = Or([42], [42, 8])
    // rule[0] = And([8, 11])

    // dbg!(&rules);
    // for (id, rule) in rules.iter() {
    //     match rule {
    //         Rule::And(a) => {
    //             if a.contains(&11) || a.contains(&8) {
    //                 println!("rule[{}] = {:?}", id, rule);
    //             }
    //         }
    //         Rule::Or(a, b) => {
    //             if a.contains(&11) || a.contains(&8) || b.contains(&11) || b.contains(&8) {
    //                 println!("rule[{}] = {:?}", id, rule);
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    index += 1;
    loop {
        if index >= lines.len() {
            break;
        }
        let line = &lines[index];

        if matches_0(&mut rules, line) {
            println!("matches: {}", line);
            res += 1;
        }
        index += 1;
    }

    // TODO: code here
    res
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

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day19/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}
