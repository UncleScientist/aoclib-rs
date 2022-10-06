use std::fmt::Display;
use std::time::{Duration, Instant};

mod aoc2015;
use aoc2015::*;

mod aoc2016;
use aoc2016::*;

pub enum Selector {
    All,
    One(usize),
    Last,
}

pub trait Runner {
    fn name(&self) -> (usize, usize);
    fn parse(&mut self);
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}

fn main() {
    let runners: Vec<fn(Selector)> = vec![run_2015, run_2016];
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 && args[1] == "all" {
        for year in runners {
            year(Selector::All);
        }
    } else if args.len() > 2 {
        let year = if let Ok(year) = args[1].parse::<usize>() {
            year
        } else {
            eprintln!("Invalid year {}", args[1]);
            std::process::exit(1);
        };

        let day = if let Ok(day) = args[2].parse::<usize>() {
            day
        } else {
            eprintln!("Invalid year {}", args[1]);
            std::process::exit(1);
        };

        if !(2015..=2021).contains(&year) {
            // if year < 2015 || year > 2021 {
            eprintln!("Year must be in range 2015..2021");
            std::process::exit(1);
        }

        runners[year - 2015](Selector::One(day));
    } else {
        runners.last().unwrap()(Selector::Last);
    }
}

pub fn run_solution<T: Runner + ?Sized>(solution: &mut T) {
    let name = solution.name();
    println!("---- {}, Day {} ----", name.0, name.1);

    let start = Instant::now();
    solution.parse();
    let parse_time = start.elapsed().as_millis();
    println!("{:3}.{:03} Parsing", parse_time / 1000, parse_time % 1000);

    let start = Instant::now();
    let p1 = solution.part1();
    let p1_time = start.elapsed();
    print_solution(1, &p1, p1_time);

    let start = Instant::now();
    let p2 = solution.part2();
    let p2_time = start.elapsed();
    print_solution(2, &p2, p2_time);
}

fn print_solution(which: usize, output: &[String], duration: Duration) {
    let ms = duration.as_millis();
    let sec_part = ms / 1000;
    let ms_part = ms % 1000;

    let mut i = output.iter();
    println!(
        "{sec_part:3}.{ms_part:03} Part {which}: {}",
        i.next().unwrap()
    );
    for line in i {
        println!("{:16}{line}", "");
    }
}
