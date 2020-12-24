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

#[derive(Debug)]
struct Input {
    foods: Vec<String>,
    alergens: Vec<String>,
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut inputs = Vec::new();
    let mut counts = HashMap::new();

    let mut f_dict = HashMap::new();
    let mut a_dict = HashMap::new();

    for line_ in lines {
        let line = line_.trim_end_matches(")");
        let parts = split_string(line, " (contains ");

        let mut foods = HashSet::new();
        let mut alergens = HashSet::new();

        let split0 = split_string(&parts[0], " ");
        for f in split0.iter() {
            if f_dict.contains_key(f) {
                foods.insert(f_dict[f]);
            } else {
                let next_id = f_dict.len() as i64;
                f_dict.insert(f.clone(), next_id);
                foods.insert(next_id);
            }
        }

        let split1 = split_string(&parts[1], ", ");
        for a in split1.iter() {
            if a_dict.contains_key(a) {
                alergens.insert(a_dict[a]);
            } else {
                let next_id = a_dict.len() as i64;
                a_dict.insert(a.clone(), next_id);
                alergens.insert(next_id);
            }
        }

        let input = Input {
            foods: split0,
            alergens: split1,
        };

        for f in input.foods.iter() {
            let key = f.clone();
            if !counts.contains_key(&key) {
                counts.insert(key, 1);
            } else {
                counts.insert(key.clone(), counts[&key] + 1);
            }
        }

        // dbg!(&input);
        inputs.push(input);
    }
    dbg!(a_dict.len());
    dbg!(f_dict.len());

    let mut a_options = HashMap::new();
    for input in inputs.iter() {
        let f_set: HashSet<String> = input.foods.iter().cloned().collect();
        for a in input.alergens.iter() {
            if !a_options.contains_key(a) {
                a_options.insert(a.clone(), f_set.clone());
            } else {
                let other_set = a_options.remove(a).unwrap();
                let new_set: HashSet<String> = other_set.intersection(&f_set).cloned().collect();
                a_options.insert(a.clone(), new_set);
            }
        }
    }

    let mut possible = HashSet::new();
    for (a, opts) in a_options.iter() {
        println!("At {} -> {:?}", a, opts);
        for opt in opts {
            possible.insert(opt);
        }
    }

    let mut res = 0;
    for (food, count) in counts.iter() {
        if !possible.contains(food) {
            res += count;
        }
    }

    res
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    // TODO: code here
    // At sesame -> {"vhkj", "lcb"}
    // At fish -> {"vhkj"}
    // At shellfish -> {"lcb", "lrqqqsg"}
    // At nuts -> {"qzlmr", "tvdvzd"}
    // At wheat -> {"qzlmr", "lrqqqsg", "shp", "vhkj"}
    // At dairy -> {"vhkj", "smfz"}
    // At peanuts -> {"vhkj", "tvdvzd"}
    // At soy -> {"dfzqlk", "vhkj"}

    let mut output = vec![
        ("sesame", "lcb"),
        ("fish", "vhkj"),
        ("shellfish", "lrqqqsg"),
        ("nuts", "qzlmr"),
        ("wheat", "shp"),
        ("dairy", "smfz"),
        ("peanuts", "tvdvzd"),
        ("soy", "dfzqlk"),
    ];

    output.sort_by_key(|x| x.0);
    println!(
        "{}",
        output.iter().map(|x| x.1).collect::<Vec<_>>().join(",")
    );

    -1
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
    let input = fs::read_to_string("input/day21/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}