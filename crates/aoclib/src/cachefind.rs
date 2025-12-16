use std::{collections::HashMap, hash::Hash};

pub struct CacheFinder<T> {
    cache: HashMap<T, usize>,
}

impl<T> Default for CacheFinder<T> {
    fn default() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

impl<T: Hash + PartialEq + Eq + Clone> CacheFinder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find(
        &mut self,
        start: &T,
        neighbors: &impl Fn(&T) -> Option<Vec<T>>,
        end_check: &impl Fn(&T) -> Option<usize>,
    ) -> usize {
        if let Some(result) = end_check(start) {
            return result;
        }

        if let Some(result) = self.cache.get(start) {
            return *result;
        }

        let count = if let Some(nodelist) = neighbors(start) {
            nodelist
                .iter()
                .map(|node| self.find(node, neighbors, end_check))
                .sum()
        } else {
            0
        };
        self.cache.insert(start.clone(), count);
        count
    }
}
