use aoclib::{Runner, Selector};

mod aoc2025_01;
/*
mod aoc2025_02;
mod aoc2025_03;
mod aoc2025_04;
mod aoc2025_05;
mod aoc2025_06;
mod aoc2025_07;
mod aoc2025_08;
mod aoc2025_09;
mod aoc2025_10;
mod aoc2025_11;
mod aoc2025_12;
mod aoc2025_13;
mod aoc2025_14;
mod aoc2025_15;
mod aoc2025_16;
mod aoc2025_17;
mod aoc2025_18;
mod aoc2025_19;
mod aoc2025_20;
mod aoc2025_21;
mod aoc2025_22;
mod aoc2025_23;
mod aoc2025_24;
mod aoc2025_25;
*/

use aoc2025_01::*;
/*
use aoc2025_02::*;
use aoc2025_03::*;
use aoc2025_04::*;
use aoc2025_05::*;
use aoc2025_06::*;
use aoc2025_07::*;
use aoc2025_08::*;
use aoc2025_09::*;
use aoc2025_10::*;
use aoc2025_11::*;
use aoc2025_12::*;
use aoc2025_13::*;
use aoc2025_14::*;
use aoc2025_15::*;
use aoc2025_16::*;
use aoc2025_17::*;
use aoc2025_18::*;
use aoc2025_19::*;
use aoc2025_20::*;
use aoc2025_21::*;
use aoc2025_22::*;
use aoc2025_23::*;
use aoc2025_24::*;
use aoc2025_25::*;
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
    /*
    let mut day02 = Aoc2025_02::new();
    let mut day03 = Aoc2025_03::new();
    let mut day04 = Aoc2025_04::new();
    let mut day05 = Aoc2025_05::new();
    let mut day06 = Aoc2025_06::new();
    let mut day07 = Aoc2025_07::new();
    let mut day08 = Aoc2025_08::new();
    let mut day09 = Aoc2025_09::new();
    let mut day10 = Aoc2025_10::new();
    let mut day11 = Aoc2025_11::new();
    let mut day12 = Aoc2025_12::new();
    let mut day13 = Aoc2025_13::new();
    let mut day14 = Aoc2025_14::new();
    let mut day15 = Aoc2025_15::new();
    let mut day16 = Aoc2025_16::new();
    let mut day17 = Aoc2025_17::new();
    let mut day18 = Aoc2025_18::new();
    let mut day19 = Aoc2025_19::new();
    let mut day20 = Aoc2025_20::new();
    let mut day21 = Aoc2025_21::new();
    let mut day22 = Aoc2025_22::new();
    let mut day23 = Aoc2025_23::new();
    let mut day24 = Aoc2025_24::new();
    let mut day25 = Aoc2025_25::new();
    */

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, /* &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
                   &mut day09, &mut day08, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
                   &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
                   &mut day22, &mut day23, &mut day24, &mut day25, */
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
