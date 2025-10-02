use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

/// Uniform Cost Search
pub fn ucs<T, S>(
    start: &T,
    neighbors: impl Fn(&T) -> Vec<(T, S)>,
    is_end: impl Fn(&T) -> bool,
) -> Option<(T, S)>
where
    T: Debug + Clone + Hash + Eq,
    S: Debug + Copy + From<u8> + Ord + Add<Output = S>,
{
    let zero: S = 0u8.into();
    let mut queue = BTreeMap::from([(zero, HashSet::from([start.clone()]))]);
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();

    dist.insert(start.clone(), Some(zero));

    while let Some((time, pos_list)) = queue.pop_first() {
        for pos in pos_list.iter() {
            if is_end(pos) {
                return Some((pos.clone(), time));
            }
            if visited.insert(pos.clone()) {
                for (new_pos, cost) in neighbors(pos) {
                    let new_time = time + cost;
                    let dist_entry = dist.entry(new_pos.clone()).or_insert(None);
                    if let Some(dist_time) = dist_entry {
                        if new_time >= *dist_time {
                            continue;
                        }

                        // Unwrap Safety: if it's in the `dist` map, then it must exist in `queue`
                        queue.get_mut(dist_time).unwrap().remove(pos);
                    }
                    *dist_entry = Some(new_time);
                    queue.entry(new_time).or_default().insert(new_pos);
                }
            }
        }
    }

    None
}

pub trait Nodes {
    fn get_value(&self, row: usize, col: usize) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub trait Searcher {
    fn moves<N: Nodes>(&self, nodes: &N) -> Vec<Self>
    where
        Self: Sized;
    fn is_win_state<N: Nodes>(&self, nodes: &N) -> bool;
    fn cost<N: Nodes>(&self, nodes: &N) -> usize;
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

        for neighbor in current.moves(nodes) {
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
