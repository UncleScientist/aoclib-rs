use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait DijkstraSearch {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized;
    fn is_win_state(&self) -> bool;
}

pub fn dijkstra_search<T: DijkstraSearch + Clone + Eq + Hash>(start: &T) -> usize {
    let mut q: HashSet<T> = HashSet::new();

    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut prev: HashMap<T, Option<T>> = HashMap::new();
    let mut index: HashMap<T, usize> = HashMap::new();
    let mut target = None;

    let mut cur = 1;
    index.insert(start.clone(), 0);
    prev.insert(start.clone(), None);
    q.insert(start.clone());

    dist.insert(start.clone(), 0);

    while !q.is_empty() {
        let u = {
            let mut best = usize::MAX;
            let mut found = None;
            for u in &q {
                let v = dist.get(u).unwrap();
                if *v < best {
                    best = *v;
                    found = Some(u.clone());
                }
            }

            found.unwrap()
        };

        if u.is_win_state() {
            target = Some(u);
            break;
        }

        if !q.remove(&u) {
            panic!("tried to remove u from q but failed");
        }

        for m in u.moves() {
            let v = if q.contains(&m) {
                m
            } else if !index.contains_key(&m) {
                index.insert(m.clone(), cur);
                cur += 1;
                dist.insert(m.clone(), usize::MAX);
                prev.insert(m.clone(), None);
                q.insert(m.clone());
                m
            } else {
                continue;
            };
            let alt = dist.get(&u).unwrap() + 1;
            let dist_v = *dist.get(&v).unwrap();
            if alt < dist_v {
                dist.insert(v.clone(), alt);
                prev.insert(v.clone(), Some(u.clone()));
            }
        }
    }

    let mut count = 0;
    let mut u = &Some(target.unwrap());
    while let Some(state) = u {
        count += 1;
        u = prev.get(state).unwrap();
    }

    count - 1
}
