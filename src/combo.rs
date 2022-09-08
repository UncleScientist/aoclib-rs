pub trait Combinations<T> {
    fn combinations(&self) -> CombinationIterator<T>;
}

impl<T: Clone> Combinations<T> for Vec<T> {
    fn combinations(&self) -> CombinationIterator<T> {
        CombinationIterator::new(self)
    }
}

pub struct CombinationIterator<T> {
    vec: Vec<T>,
    current: u128,
}

impl<T: Clone> CombinationIterator<T> {
    fn new(vec: &[T]) -> Self {
        Self {
            vec: Vec::from(vec),
            current: (1 << vec.len()) - 1,
        }
    }
}

impl<T: Clone> Iterator for CombinationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.current == 0 {
            return None;
        }

        let mut vec = Vec::new();
        let mut num = self.current;
        let mut index = 0;
        self.current -= 1;
        while num != 0 {
            if (num & 1) == 1 {
                vec.push(self.vec[index].clone());
            }

            index += 1;
            num >>= 1;
        }

        Some(vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn combos_of_three_items() {
        let v = vec!['A', 'B', 'C'];

        let mut hs = HashSet::from([
            vec!['A'],
            vec!['B'],
            vec!['C'],
            vec!['A', 'B'],
            vec!['A', 'C'],
            vec!['B', 'C'],
            vec!['A', 'B', 'C'],
        ]);
        for entry in v.combinations() {
            assert!(hs.contains(&entry));
            hs.remove(&entry);
        }

        assert!(hs.is_empty());
    }

    #[test]
    fn single_entry_vec() {
        let v = vec![1];
        let mut iter = v.combinations();
        assert_eq!(iter.next().unwrap(), vec![1]);
        assert!(iter.next().is_none());
    }

    #[test]
    fn empty_vec() {
        let v: Vec<i32> = Vec::new();
        assert!(v.combinations().next().is_none());
    }
}
