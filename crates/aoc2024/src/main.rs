use aoclib::{Runner, Selector};

mod aoc2024_01;
mod aoc2024_02;
mod aoc2024_03;
mod aoc2024_04;
mod aoc2024_05;
mod aoc2024_06;
mod aoc2024_07;
mod aoc2024_08;
mod aoc2024_09;
mod aoc2024_10;
mod aoc2024_11;
// mod aoc2024_12;
// mod aoc2024_13;
// mod aoc2024_14;
// mod aoc2024_15;
// mod aoc2024_16;
// mod aoc2024_17;
// mod aoc2024_18;
// mod aoc2024_19;
// mod aoc2024_20;
// mod aoc2024_21;
// mod aoc2024_22;
// mod aoc2024_23;

use aoc2024_01::*;
use aoc2024_02::*;
use aoc2024_03::*;
use aoc2024_04::*;
use aoc2024_05::*;
use aoc2024_06::*;
use aoc2024_07::*;
use aoc2024_08::*;
use aoc2024_09::*;
use aoc2024_10::*;
use aoc2024_11::*;
// use aoc2024_12::*;
// use aoc2024_13::*;
// use aoc2024_14::*;
// use aoc2024_15::*;
// use aoc2024_16::*;
// use aoc2024_17::*;
// use aoc2024_18::*;
// use aoc2024_19::*;
// use aoc2024_20::*;
// use aoc2024_21::*;
// use aoc2024_22::*;
// use aoc2024_23::*;

fn main() {
    run_2024(Selector::Last);
}

fn run_2024(which: Selector) {
    let mut day01 = Aoc2024_01::new();
    let mut day02 = Aoc2024_02::new();
    let mut day03 = Aoc2024_03::new();
    let mut day04 = Aoc2024_04::new();
    let mut day05 = Aoc2024_05::new();
    let mut day06 = Aoc2024_06::new();
    let mut day07 = Aoc2024_07::new();
    let mut day08 = Aoc2024_08::new();
    let mut day09 = Aoc2024_09::new();
    let mut day10 = Aoc2024_10::new();
    let mut day11 = Aoc2024_11::new();
    //     let mut day12 = Aoc2024_12::new();
    //     let mut day13 = Aoc2024_13::new();
    //     let mut day14 = Aoc2024_14::new();
    //     let mut day15 = Aoc2024_15::new();
    //     let mut day16 = Aoc2024_16::new();
    //     let mut day17 = Aoc2024_17::new();
    //     let mut day18 = Aoc2024_18::new();
    //     let mut day19 = Aoc2024_19::new();
    //     let mut day20 = Aoc2024_20::new();
    //     let mut day21 = Aoc2024_21::new();
    //     let mut day22 = Aoc2024_22::new();
    //     let mut day23 = Aoc2024_23::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day09, &mut day08, &mut day10,
        &mut day11,
        // &mut day14, &mut day13, &mut day12,
        // &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
        // &mut day22, &mut day23,
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
