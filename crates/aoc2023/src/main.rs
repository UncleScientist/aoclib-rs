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

    let mut days: Vec<&mut dyn Runner> = vec![
        &mut day01, &mut day02, &mut day03, &mut day04, &mut day05, &mut day06, &mut day07,
        &mut day08, &mut day09, &mut day10,
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
