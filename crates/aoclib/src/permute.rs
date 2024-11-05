pub trait Permutations<T> {
    fn permutations(&self) -> PermutationIterator<T>;
}

impl<T: Clone> Permutations<T> for Vec<T> {
    fn permutations(&self) -> PermutationIterator<T> {
        PermutationIterator::new(self)
    }
}

pub struct PermutationIterator<T> {
    i: usize,
    c: Vec<usize>,
    a: Vec<T>,
}

impl<T: Clone> PermutationIterator<T> {
    pub fn new(vec: &[T]) -> Self {
        let mut c: Vec<usize> = Vec::with_capacity(vec.len());
        (0..vec.len()).for_each(|_| c.push(0));

        Self {
            c,
            i: 0,
            a: vec.into(),
        }
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.a.clone());
        }

        while self.i < self.a.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.a.swap(0, self.i);
                } else {
                    self.a.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 1;
                return Some(self.a.clone());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_first_one() {
        let v = vec![1, 2, 3];
        let mut p = PermutationIterator::new(&v);
        assert_eq!(p.next(), Some(vec![1, 2, 3]));
    }
}
