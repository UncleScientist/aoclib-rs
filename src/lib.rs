use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

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
