use std::fmt::Debug;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug)]
enum ParsePointError {}

#[derive(Debug, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr> Point<T> {
    pub fn parse(s: &str, separator: &str) -> Self
    where
        <T as FromStr>::Err: Debug,
    {
        let mut value = s.split(separator);
        let x = value.next().unwrap().parse::<T>().unwrap();
        let y = value.next().unwrap().parse::<T>().unwrap();

        Self { x, y }
    }
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = String;

    fn from_str(p: &str) -> Result<Self, Self::Err> {
        let (x, y) = p.split_once(", ").ok_or("input file corrupt")?;
        Ok(Self {
            x: x.parse().map_err(|_| "x value corrupt")?,
            y: y.parse().map_err(|_| "y value corrupt")?,
        })
    }
}

impl<T: Copy + Ord + Add<T, Output = T> + Sub<T, Output = T>> Point<T> {
    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: Ord::min(self.x, other.x),
            y: Ord::min(self.y, other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: Ord::max(self.x, other.x),
            y: Ord::max(self.y, other.y),
        }
    }

    pub fn dist(&self, x: T, y: T) -> T
    where
        <T as Sub>::Output: Add<T>,
    {
        let delta_x = if self.x > x { self.x - x } else { x - self.x };
        let delta_y = if self.y > y { self.y - y } else { y - self.y };

        delta_x + delta_y
    }
}
