use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr> Point<T> {
    pub fn parse(s: &str) -> Self
    where
        <T as FromStr>::Err: Debug,
    {
        let mut value = s.split(',');
        let x = value.next().unwrap().parse::<T>().unwrap();
        let y = value.next().unwrap().parse::<T>().unwrap();

        Self { x, y }
    }
}
