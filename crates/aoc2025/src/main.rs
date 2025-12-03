use aoclib::{Runner, Selector};

mod aoc2025_01;
mod aoc2025_02;
mod aoc2025_03;
/*
mod aoc2025_04;
mod aoc2025_05;
mod aoc2025_06;
mod aoc2025_07;
mod aoc2025_08;
mod aoc2025_09;
mod aoc2025_10;
mod aoc2025_11;
mod aoc2025_12;
*/

use aoc2025_01::*;
use aoc2025_02::*;
use aoc2025_03::*;
/*
use aoc2025_04::*;
use aoc2025_05::*;
use aoc2025_06::*;
use aoc2025_07::*;
use aoc2025_08::*;
use aoc2025_09::*;
use aoc2025_10::*;
use aoc2025_11::*;
use aoc2025_12::*;
*/

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        run_2025(Selector::Last);
    } else if let Ok(day) = args[1].parse::<usize>() {
        run_2025(Selector::One(day));
    } else if args[1] == "-a" || args[1] == "--all" {
        run_2025(Selector::All);
    }
}

fn run_2025(which: Selector) {
    let mut day01 = Aoc2025_01::new();
    let mut day02 = Aoc2025_02::new();
    let mut day03 = Aoc2025_03::new();
    /*
    let mut day04 = Aoc2025_04::new();
    let mut day05 = Aoc2025_05::new();
    let mut day06 = Aoc2025_06::new();
    let mut day07 = Aoc2025_07::new();
    let mut day08 = Aoc2025_08::new();
    let mut day09 = Aoc2025_09::new();
    let mut day10 = Aoc2025_10::new();
    let mut day11 = Aoc2025_11::new();
    let mut day12 = Aoc2025_12::new();
    */

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02,
        &mut day03, /* &mut day04, &mut day05, &mut day06, &mut day07,
                   &mut day09, &mut day08, &mut day10, &mut day11, &mut day12, */
    ];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            aoclib::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                aoclib::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            aoclib::run_solution(*d);
        }
    }
}
