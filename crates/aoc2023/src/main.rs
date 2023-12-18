use aoclib::{Runner, Selector};

mod aoc2023_01;
mod aoc2023_02;
mod aoc2023_03;
mod aoc2023_04;
mod aoc2023_05;
mod aoc2023_06;
mod aoc2023_07;
mod aoc2023_08;
mod aoc2023_09;
mod aoc2023_10;
mod aoc2023_11;
mod aoc2023_12;
mod aoc2023_13;
mod aoc2023_14;
mod aoc2023_15;
mod aoc2023_16;
mod aoc2023_17;
mod aoc2023_18;

use aoc2023_01::*;
use aoc2023_02::*;
use aoc2023_03::*;
use aoc2023_04::*;
use aoc2023_05::*;
use aoc2023_06::*;
use aoc2023_07::*;
use aoc2023_08::*;
use aoc2023_09::*;
use aoc2023_10::*;
use aoc2023_11::*;
use aoc2023_12::*;
use aoc2023_13::*;
use aoc2023_14::*;
use aoc2023_15::*;
use aoc2023_16::*;
use aoc2023_17::*;
use aoc2023_18::*;

fn main() {
    run_2023(Selector::Last);
}

fn run_2023(which: Selector) {
    let mut day01 = Aoc2023_01::new();
    let mut day02 = Aoc2023_02::new();
    let mut day03 = Aoc2023_03::new();
    let mut day04 = Aoc2023_04::new();
    let mut day05 = Aoc2023_05::new();
    let mut day06 = Aoc2023_06::new();
    let mut day07 = Aoc2023_07::new();
    let mut day08 = Aoc2023_08::new();
    let mut day09 = Aoc2023_09::new();
    let mut day10 = Aoc2023_10::new();
    let mut day11 = Aoc2023_11::new();
    let mut day12 = Aoc2023_12::new();
    let mut day13 = Aoc2023_13::new();
    let mut day14 = Aoc2023_14::new();
    let mut day15 = Aoc2023_15::new();
    let mut day16 = Aoc2023_16::new();
    let mut day17 = Aoc2023_17::new();
    let mut day18 = Aoc2023_18::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day14, &mut day13, &mut day12,
        &mut day15, &mut day16, &mut day17, &mut day18,
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
