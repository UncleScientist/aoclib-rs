use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Searcher {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized;
    fn is_win_state(&self) -> bool;
}

pub fn dijkstra_search<T: Searcher + Clone + Eq + Hash>(start: &T) -> Option<(T, usize)> {
    let mut q: HashSet<T> = HashSet::new();

    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut index: HashSet<T> = HashSet::new();
    let mut target = None;

    index.insert(start.clone());
    q.insert(start.clone());

    dist.insert(start.clone(), 0);

    while !q.is_empty() {
        let u = q
            .iter()
            .map(|item| (item, dist.get(item).unwrap()))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .clone();

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
            } else if index.insert(m.clone()) {
                dist.insert(m.clone(), usize::MAX);
                q.insert(m.clone());
                m
            } else {
                continue;
            };
            let alt = dist.get(&u).unwrap() + 1;
            let dist_v = *dist.get(&v).unwrap();
            if alt < dist_v {
                dist.insert(v.clone(), alt);
            }
        }
    }

    let target = target?;

    Some((target.clone(), *dist.get(&target)?))
}
