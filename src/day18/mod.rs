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

extern crate aoc;
use aoc::*;

pub fn eval(s: &Vec<char>, start: usize, end: usize, prev: &Vec<usize>) -> i64 {
    // println!("  eval from {:?}", s[start..=end].iter().collect::<String>());
    let mut value = 0;
    let mut next = 0;
    if s[end] == ')' {
        value = eval(s, prev[end] + 1, end - 1, prev);
        if prev[end] == start {
            return value;
        }
        next = prev[end] - 1;
    } else {
        assert!(s[end].is_digit(10), "{}", s[end]);
        value = s[end].to_digit(10).unwrap() as i64;
        if end == start {
            return value;
        }
        next = end - 1;
    }
    let op = s[next];
    assert!(op == '*' || op == '+', "op = {}", op);
    let next_value = eval(s, start, next - 1, prev);
    let res = {
        match op {
            '*' => next_value * value,
            '+' => next_value + value,
            _ => unreachable!(),
        }
    };
    // println!(
    //     "  eval from {:?} = {}",
    //     s[start..=end].iter().collect::<String>(),
    //     res
    // );
    res
}

pub fn eval_full(s: &Vec<char>) -> i64 {
    let mut q = VecDeque::new();
    let mx = s.len();
    let mut next = vec![mx; s.len()];
    let mut prev = vec![mx; s.len()];

    for i in 0..s.len() {
        if s[i] == '(' {
            q.push_back(i);
        }
        if s[i] == ')' {
            let index = q.pop_back().unwrap();
            next[index] = i;
            prev[i] = index;
        }
    }
    eval(s, 0, s.len() - 1, &prev)
}

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for _line in lines {
        let line = _line.trim().replace(" ", "");
        // println!("eval from {}", line);
        let s = to_v_char(&line);
        let value = eval_full(&s);
        // println!(" value = {}", value);
        res += value;
        // break;
    }

    // TODO: code here
    res
}

#[derive(Debug)]
pub struct Tokens {
    values: Vec<i64>,
    ops: Vec<char>,
}

pub fn parse_tokens(s: &Vec<char>, start: usize, end: usize, prev: &Vec<usize>) -> Tokens {
    let (value, go) = eat_token(s, start, end, prev);
    if go == start {
        return Tokens {
            values: vec![value],
            ops: Vec::new(),
        };
    }
    let op = s[go - 1];
    assert!(op == '*' || op == '+', "op = {}", op);
    let mut tokens = parse_tokens(s, start, go - 2, prev);
    tokens.values.push(value);
    tokens.ops.push(op);
    tokens
}

pub fn eat_token(s: &Vec<char>, start: usize, end: usize, prev: &Vec<usize>) -> (i64, usize) {
    let mut value = 0;
    let mut go = 0;
    if s[end] == ')' {
        value = eval2(s, prev[end] + 1, end - 1, prev);
        go = prev[end];
    } else {
        assert!(s[end].is_digit(10), "{}", s[end]);
        value = s[end].to_digit(10).unwrap() as i64;
        go = end;
    }
    (value, go)
}

pub fn eval_tokens(tokens: &Tokens) -> i64 {
    assert_eq!(tokens.ops.len() + 1, tokens.values.len(), "{:?}", tokens);
    let mut q = VecDeque::new();
    q.push_back(tokens.values[0]);
    for i in 0..tokens.ops.len() {
        // println!("i = {}, q = {:?}", i, q);
        let v = tokens.values[i + 1];
        if tokens.ops[i] == '*' {
            q.push_back(v);
        } else {
            let now = q.pop_back().unwrap();
            q.push_back(now + v);
        }
    }
    let mut res = 1;
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        res *= v;
    }
    // println!("   eval tokens {:?} = {}", tokens, res);
    res
}

pub fn eval2(s: &Vec<char>, start: usize, end: usize, prev: &Vec<usize>) -> i64 {
    // println!(
    //     "  eval from {:?}",
    //     s[start..=end].iter().collect::<String>()
    // );
    let (value, go) = eat_token(s, start, end, prev);
    if go == start {
        return value;
    }
    let op = s[go - 1];
    assert!(op == '*' || op == '+', "op = {}", op);
    let mut tokens = parse_tokens(s, start, go - 2, prev);
    tokens.values.push(value);
    tokens.ops.push(op);
    let res = eval_tokens(&tokens);
    // println!(
    //     "  eval from {:?} = {}",
    //     s[start..=end].iter().collect::<String>(),
    //     res
    // );
    res
}

pub fn eval_full2(s: &Vec<char>) -> i64 {
    let mut q = VecDeque::new();
    let mx = s.len();
    let mut next = vec![mx; s.len()];
    let mut prev = vec![mx; s.len()];

    for i in 0..s.len() {
        if s[i] == '(' {
            q.push_back(i);
        }
        if s[i] == ')' {
            let index = q.pop_back().unwrap();
            next[index] = i;
            prev[i] = index;
        }
    }
    eval2(s, 0, s.len() - 1, &prev)
}

pub fn part2(lines: &Vec<String>) -> i64 {
    let mut res = 0;
    for _line in lines {
        let line = _line.trim().replace(" ", "");
        let s = to_v_char(&line);
        let value = eval_full2(&s);
        // println!("eval from {} --> {}", line, value);
        // println!(" value = {}", value);
        res += value;
        // break;
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
        assert_eq!(part1(&lines), 21347713555555);
    }

    #[test]
    fn test_part2() {
        let lines = read_main_input();
        assert_eq!(part2(&lines), 275011754427339);
    }
}

pub fn read_main_input() -> Vec<String> {
    let input = fs::read_to_string("input/day18/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    to_lines(&input)
}

pub fn read_input_from_args(args: &Vec<String>) -> Vec<String> {
    println!("args: {:?}", args);
    if args.len() <= 1 {
        return read_main_input();
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    to_lines(&input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_input_from_args(&args);

    println!("part1 = {}", part1(&lines));
    println!("part2 = {}", part2(&lines));
}
