use std::{
    cmp::{PartialEq, PartialOrd},
    convert::From,
    ops::{Add, AddAssign, Sub},
};

pub struct CoinChange<T> {
    coins: Vec<Option<T>>,
    change: Vec<Option<T>>,
}

impl<T> CoinChange<T>
where
    T: Copy
        + Clone
        + From<usize>
        + PartialEq
        + PartialOrd
        + Sub
        + AddAssign
        + Ord
        + Add<Output = T>,
    usize: From<T> + From<<T as Sub>::Output>,
{
    pub fn new(coins: &[T]) -> Self {
        Self {
            coins: coins.iter().copied().map(|c| Some(c)).collect(),
            change: Vec::new(),
        }
    }

    pub fn min_coins(&mut self, amount: T) -> Option<T> {
        let zero: T = 0_usize.into();
        let one: T = 1_usize.into();

        let index: usize = amount.into();

        if amount == zero {
            return Some(zero);
        }

        if self.change.len() <= 1 + index {
            self.change.resize(1 + index, None);
        }
        self.change[0] = Some(zero);

        let mut i = one;
        while i <= amount {
            for coin in 0..self.coins.len() {
                if let Some(value) = self.coins[coin]
                    && value <= i
                {
                    let first: usize = (i - value).into();
                    let second: usize = i.into();
                    if let Some(cur_count) = self.change[first] {
                        if let Some(old_value) = self.change[second] {
                            let ov: T = old_value;
                            self.change[second] = Some(ov.min(cur_count + one))
                        } else {
                            self.change[second] = Some(cur_count + one);
                        }
                    }
                }
            }
            i += one;
        }

        self.change[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn failed_test() {
        let mut cc = CoinChange::new(&[186, 419, 83, 408]);
        assert_eq!(Some(20), cc.min_coins(6249));
    }

    #[test]
    fn ex1() {
        let mut cc = CoinChange::new(&[1, 2, 5]);
        assert_eq!(Some(3), cc.min_coins(11));
    }

    #[test]
    fn ex1a() {
        let mut cc = CoinChange::new(&[1, 2, 5]);
        assert_eq!(Some(20), cc.min_coins(100));
    }

    #[test]
    fn ex2() {
        let mut cc = CoinChange::new(&[2]);
        assert_eq!(None, cc.min_coins(3));
    }

    #[test]
    fn ex3() {
        let mut cc = CoinChange::new(&[1]);
        assert_eq!(Some(0), cc.min_coins(0));
    }
}
