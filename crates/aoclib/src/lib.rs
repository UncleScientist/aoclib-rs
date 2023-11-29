use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};

mod point;
pub use point::*;

mod permute;
pub use permute::*;

mod combo;
pub use combo::*;

mod mathemagic;
pub use mathemagic::*;

mod search;
pub use search::*;

pub fn read_to_chars<T: AsRef<Path>>(pathname: T) -> Vec<char> {
    let data = read_to_string(pathname).expect("unable to open file");
    data.chars().collect()
}

pub fn numbers<T: AsRef<Path>, U: FromStr>(pathname: T, sep: char) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    let data = read_to_string(pathname).expect("unable to open file");
    let mut result = Vec::new();

    for line in data.split('\n') {
        if !line.is_empty() {
            let iter = line.split(sep);
            result.push(
                iter.map(|x| x.parse::<U>().expect("unable to parse number"))
                    .collect::<Vec<U>>(),
            );
        }
    }

    result
}

pub fn read_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

pub fn read_num_records<T: AsRef<Path>, U: FromStr>(pathname: T) -> Vec<Vec<U>>
where
    <U as FromStr>::Err: Debug,
{
    read_to_string(pathname)
        .expect("unable to open file")
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|num| num.parse::<U>().expect("unable to parse number"))
                .collect::<Vec<U>>()
        })
        .collect()
}

pub fn read_single_line<T: AsRef<Path>>(pathname: T) -> Vec<char> {
    read_to_string(pathname)
        .expect("unable to open file")
        .chars()
        .filter(|&ch| ch != '\n')
        .collect()
}

pub fn read_numbers<T: AsRef<Path> + Debug, U: FromStr>(pathname: T) -> Vec<U>
where
    <U as FromStr>::Err: Debug,
{
    let mut result = Vec::new();
    for line in std::fs::read_to_string(pathname)
        .expect("Unable to find {pathname:?}")
        .lines()
    {
        result.push(line.parse::<U>().unwrap());
    }
    result
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}

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
