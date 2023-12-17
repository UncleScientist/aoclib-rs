use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Nodes {
    fn get_value(&self, row: usize, col: usize) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub trait Searcher {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized;
    fn is_win_state<N: Nodes>(&self, nodes: &N) -> bool;
    fn cost<N: Nodes>(&self, nodes: &N) -> usize;
}

pub fn dijkstra_search<N: Nodes, T: Searcher + Clone + Eq + Hash>(
    start: &T,
    nodes: &N,
) -> Option<(T, usize)> {
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

        if u.is_win_state(nodes) {
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

pub fn astar_search<N: Nodes, T: Searcher + Clone + Eq + Hash, H: Fn(&T) -> usize>(
    start: &T,
    heuristic: H,
    nodes: &N,
) -> Option<(T, usize)> {
    let mut open_set = HashSet::new();
    open_set.insert(start.clone());

    let mut gscore = HashMap::new();
    gscore.insert(start.clone(), 0);

    let mut fscore = HashMap::new();
    fscore.insert(start.clone(), heuristic(start));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by(|a, b| fscore.get(a).unwrap().cmp(fscore.get(b).unwrap()))
            .unwrap()
            .clone();
        if current.is_win_state(nodes) {
            let dist = *gscore.get(&current).unwrap();
            return Some((current, dist));
        }

        open_set.remove(&current);

        for neighbor in current.moves() {
            let tentative_gscore = gscore.get(&current).unwrap() + neighbor.cost(nodes);
            if tentative_gscore < *gscore.entry(neighbor.clone()).or_insert(usize::MAX) {
                gscore.insert(neighbor.clone(), tentative_gscore);
                fscore.insert(neighbor.clone(), tentative_gscore + heuristic(&neighbor));
                open_set.insert(neighbor);
            }
        }
    }

    None
}
