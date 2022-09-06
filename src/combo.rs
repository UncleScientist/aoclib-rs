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
    combo: usize,
    idx: Vec<usize>,
    first: bool,
}

impl<T: Clone> CombinationIterator<T> {
    fn new(vec: &[T]) -> Self {
        Self {
            vec: Vec::from(vec),
            combo: 1,
            idx: Vec::new(),
            first: true,
        }
    }

    fn combos(&self) -> Vec<T> {
        self.idx.iter().map(|&idx| self.vec[idx].clone()).collect()
    }
}

impl<T: Clone> Iterator for CombinationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            if self.first {
                self.idx = Vec::new();
                for idx in 0..self.combo {
                    self.idx.push(idx);
                }
                self.first = false;
                return Some(self.combos());
            }

            let mut done = false;
            while !done {
                done = true;
                'inc: for inc_index in (0..self.combo).rev() {
                    self.idx[inc_index] += 1;
                    if self.idx[inc_index] == self.vec.len() {
                        continue;
                    }

                    for next in inc_index + 1..self.combo {
                        self.idx[next] = self.idx[next - 1] + 1;
                        if self.idx[next] == self.vec.len() {
                            continue 'inc;
                        }
                    }

                    done = false;
                    break;
                }

                if !done {
                    return Some(self.combos());
                }
            }

            self.combo += 1;
            self.first = true;

            if self.combo > self.vec.len() {
                return None;
            }
        }
    }
}
