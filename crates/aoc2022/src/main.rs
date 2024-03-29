use aoclib::{Runner, Selector};

mod aoc2022_01;
mod aoc2022_02;
mod aoc2022_03;
mod aoc2022_04;
mod aoc2022_05;
mod aoc2022_06;
mod aoc2022_07;
mod aoc2022_08;
mod aoc2022_09;
mod aoc2022_10;
mod aoc2022_11;
mod aoc2022_12;
mod aoc2022_13;
mod aoc2022_14;
mod aoc2022_15;
mod aoc2022_16;
mod aoc2022_17;
mod aoc2022_18;
mod aoc2022_19;
mod aoc2022_20;
mod aoc2022_21;
mod aoc2022_22;
mod aoc2022_23;
mod aoc2022_24;
mod aoc2022_25;

use aoc2022_01::*;
use aoc2022_02::*;
use aoc2022_03::*;
use aoc2022_04::*;
use aoc2022_05::*;
use aoc2022_06::*;
use aoc2022_07::*;
use aoc2022_08::*;
use aoc2022_09::*;
use aoc2022_10::*;
use aoc2022_11::*;
use aoc2022_12::*;
use aoc2022_13::*;
use aoc2022_14::*;
use aoc2022_15::*;
use aoc2022_16::*;
use aoc2022_17::*;
use aoc2022_18::*;
use aoc2022_19::*;
use aoc2022_20::*;
use aoc2022_21::*;
use aoc2022_22::*;
use aoc2022_23::*;
use aoc2022_24::*;
use aoc2022_25::*;

fn main() {
    run_2022(Selector::All);
}

fn run_2022(which: Selector) {
    let mut day01 = Aoc2022_01::new();
    let mut day02 = Aoc2022_02::new();
    let mut day03 = Aoc2022_03::new();
    let mut day04 = Aoc2022_04::new();
    let mut day05 = Aoc2022_05::new();
    let mut day06 = Aoc2022_06::new();
    let mut day07 = Aoc2022_07::new();
    let mut day08 = Aoc2022_08::new();
    let mut day09 = Aoc2022_09::new();
    let mut day10 = Aoc2022_10::new();
    let mut day11 = Aoc2022_11::new();
    let mut day12 = Aoc2022_12::new();
    let mut day13 = Aoc2022_13::new();
    let mut day14 = Aoc2022_14::new();
    let mut day15 = Aoc2022_15::new();
    let mut day16 = Aoc2022_16::new();
    let mut day17 = Aoc2022_17::new();
    let mut day18 = Aoc2022_18::new();
    let mut day19 = Aoc2022_19::new();
    let mut day20 = Aoc2022_20::new();
    let mut day21 = Aoc2022_21::new();
    let mut day22 = Aoc2022_22::new();
    let mut day23 = Aoc2022_23::new();
    let mut day24 = Aoc2022_24::new();
    let mut day25 = Aoc2022_25::new();

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10, &mut day11, &mut day12, &mut day13, &mut day14,
        &mut day15, &mut day16, &mut day17, &mut day18, &mut day19, &mut day20, &mut day21,
        &mut day22, &mut day23, &mut day24, &mut day25,
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
