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

fn compute_score(p: &VecDeque<i64>) -> i64 {
    let mut res = 0;
    for (index, c) in p.iter().rev().enumerate() {
        res += (index + 1) as i64 * c;
    }
    res
}

pub fn part1(lines: &String) -> i64 {
    let mut res = 0;
    // TODO: code here
    let players = split_string(lines, "\n\n");

    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();

    let cards = split_string(&players[0].trim(), "\n");
    for card in cards.iter().skip(1) {
        p1.push_back(parse_i64(card));
    }

    let cards = split_string(&players[1].trim(), "\n");
    for card in cards.iter().skip(1) {
        p2.push_back(parse_i64(card));
    }

    dbg!(p1.len());
    dbg!(p2.len());

    let mut round = 0;
    loop {
        round += 1;
        if p1.is_empty() || p2.is_empty() {
            break;
        }
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        assert!(c1 != c2);
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    dbg!(round);
    let total = compute_score(&p1) + compute_score(&p2);
    total
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Winner {
    P1,
    P2,
}

// Before either player deals a card,
//   if there was a previous round in this game that had exactly
//   the same cards in the same order in the same players' decks,
//   the game instantly ends in a win for player 1.
//   Previous rounds from other games are not considered.
//   (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
// Otherwise, this round's cards must be in a new configuration;
//    the players begin the round by each drawing the top card of their deck as normal.
// If both players have at least as many
//    cards remaining in their deck as the value of the card they just drew,
//    the winner of the round is determined by playing a new game of Recursive Combat (see below).
// Otherwise, at least one player must not have enough cards left in their deck to recurse;
//    the winner of the round is the player with the higher-value card.

type Game = (VecDeque<i64>, VecDeque<i64>);

fn play_recursive(
    p1: &mut VecDeque<i64>,
    p2: &mut VecDeque<i64>,
    depth: usize,
    cache: &mut HashMap<Game, Winner>,
) -> Winner {
    // if depth >= 1 {
    //     return Winner::P1;
    // }
    if depth <= 10 {
        // let indent = ">>".repeat(depth);
        // println!("");
        // println!("  {} play recurisve depth = {}, cache = {}", indent, depth, cache.len());
        // println!("  {} p1 ({}): {:?}", indent, p1.len(), p1);
        // println!("  {} p2 ({}): {:?}", indent, p2.len(), p2);
    }
    // if depth >= 5 {
    //     return Winner::P1;
    // }
    // let mut p1 = p1_;
    // let mut p2 = p2_;
    let total = p1.len() + p2.len();
    let p1_max = *p1.iter().max().unwrap();
    let p2_max = *p2.iter().max().unwrap();

    let mut all_values: Vec<_> = p1.iter().chain(p2.iter()).collect();
    all_values.sort();
    let m0 = all_values[0];
    let m1 = all_values[1];
    if *m1 > total as i64 - 2 {
        if p1_max > p2_max {
            return Winner::P1;
        }
    }

    let init_key = (p1.clone(), p2.clone());
    if cache.contains_key(&init_key) {
        return cache[&init_key];
    }
    let indent = ">>".repeat(depth);

    let mut was_before = HashSet::new();
    let mut round = 0;
    let mut recursive = 0;
    let winner = {
        loop {
            if false && depth == 0 {
                println!("");
                println!("depth={} playing round={}", depth, round);
                println!("  p1 ({}): {:?}", p1.len(), p1);
                println!("  p2 ({}): {:?}", p2.len(), p2);
            }
            let key = (p1.clone(), p2.clone());
            // if cache.contains_key(&key) {
            //     break cache[&key];
            // }
            // if depth >= 4 && round > 100 {
            //     break Winner::P1;
            // }

            // if round > 500 {
            //     break Winner::P1;
            // }

            if was_before.contains(&key) {
                break Winner::P1;
            } else {
                was_before.insert(key);
            }

            if p1.is_empty() {
                break Winner::P2;
            }
            if p2.is_empty() {
                break Winner::P1;
            }

            let c1 = p1.pop_front().unwrap();
            let c2 = p2.pop_front().unwrap();

            let winner = {
                let enable_recursive = true;
                if enable_recursive && c1 <= p1.len() as i64 && c2 <= p2.len() as i64 {
                    let mut p1_: VecDeque<i64> = p1.iter().take(c1 as usize).copied().collect();
                    let mut p2_: VecDeque<i64> = p2.iter().take(c2 as usize).copied().collect();
                    if false && depth <= 0 {
                        println!("");
                        println!(
                            "  {} depth={} round={} recursive at {} vs {}, cache={}",
                            indent,
                            depth,
                            round,
                            c1,
                            c2,
                            cache.len()
                        );
                        println!("  {} p1 ({}): {:?}", indent, p1.len(), p1);
                        println!("  {} p2 ({}): {:?}", indent, p2.len(), p2);
                    }
                    recursive += 1;
                    // let winner = Winner::P1;
                    // if depth == 0 && recursive == 2 {
                    //     Winner::P2
                    // } else {
                    //     Winner::P1
                    // }
                    let winner = play_recursive(&mut p1_, &mut p2_, depth + 1, cache);
                    winner
                } else {
                    if c1 > c2 {
                        Winner::P1
                    } else {
                        Winner::P2
                    }
                }
            };
            if winner == Winner::P1 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
            round += 1;
        }
    };
    // if winner == Winner::P2 {
    //     println!(
    //         "++  {} at depth={}, rounds={}, recursive={}, winner={:?}",
    //         indent, depth, round, recursive, winner
    //     );
    //     println!("++  {} p1 ({}): {:?}", indent, init_key.0.len(), init_key.0);
    //     println!("++  {} p2 ({}): {:?}", indent, init_key.1.len(), init_key.1);
    //     // assert!(false);
    // }

    // cache.insert(init_key, winner);
    // for key in was_before.iter() {
    //     cache.insert(key.clone(), winner);
    // }
    return winner;
}

pub fn part2(lines: &String) -> i64 {
    let players = split_string(lines, "\n\n");

    let mut p1 = VecDeque::new();
    let mut p2 = VecDeque::new();

    let cards = split_string(&players[0].trim(), "\n");
    for card in cards.iter().skip(1) {
        p1.push_back(parse_i64(card));
    }

    let cards = split_string(&players[1].trim(), "\n");
    for card in cards.iter().skip(1) {
        p2.push_back(parse_i64(card));
    }

    dbg!(p1.len());
    dbg!(p2.len());

    let mut cache = HashMap::new();
    let winner = play_recursive(&mut p1, &mut p2, 0, &mut cache);
    dbg!(cache.len());
    if winner == Winner::P1 {
        compute_score(&p1)
    } else {
        compute_score(&p2)
    }
}

// depth=0 playing round=347
//   p1 (50): [10, 9, 13, 12, 46, 26, 34, 6, 44, 23, 50, 27, 41, 25, 40, 38, 39, 37, 48, 21, 36, 2, 18, 5, 49, 22, 31, 19, 29, 16, 28, 7, 43, 17, 30, 14, 11, 3, 15, 8, 47, 42, 45, 32, 24, 20, 35, 4, 33, 1]
//   p2 (0): []

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let lines = read_main_input();
    //     assert_eq!(part1(&lines), -1);
    // }

    // #[test]
    // fn test_part2() {
    //     let lines = read_main_input();
    //     assert_eq!(part2(&lines), -1);
    // }
}

pub fn read_main_input() -> String {
    let input = fs::read_to_string("input/day22/in.txt").unwrap();
    // let input = fs::read_to_string("input/day07/demo.txt").unwrap();
    // to_lines(&input)
    input
}
