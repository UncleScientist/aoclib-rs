use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_16 {
    tunnels: HashMap<String, Valve>,
    shortcuts: HashMap<String, HashMap<String, usize>>,
}

impl Aoc2022_16 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_16 {
    fn name(&self) -> (usize, usize) {
        (2022, 16)
    }

    fn parse(&mut self) {
        // let lines = aoclib::read_lines("test-input.txt");
        let lines = aoclib::read_lines("input/2022-16.txt");
        for line in lines {
            let (v, t) = line.split_once(';').unwrap();
            let (vname, rate) = v.split_once('=').unwrap();
            let name = vname.split(' ').nth(1).unwrap();

            let allpaths = t.split(", ").collect::<Vec<&str>>();
            let first = allpaths[0].split(' ').last().unwrap();
            let first = first.split(' ').last().unwrap();

            let mut paths = vec![first.to_string()];
            paths.extend(allpaths.iter().skip(1).map(|s| s.to_string()));

            self.tunnels.insert(
                name.to_string(),
                Valve {
                    name: name.to_string(),
                    flow_rate: rate.parse().unwrap(),
                    paths,
                },
            );
        }

        for v in self.tunnels.keys() {
            self.shortcuts
                .insert(v.clone(), shortcuts(v, &self.tunnels));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut search = Search::default();
        let walker = Walk {
            loc: "AA".to_string(),
            remaining_time: 30,
            helper: false,
            turned_on: HashSet::new(),
        };
        aoclib::output(search.bfs(&walker, &self.tunnels, &self.shortcuts))
        //aoclib::output("unsolved")
    }

    fn part2(&mut self) -> Vec<String> {
        let mut search = Search::default();
        let walker = Walk {
            loc: "AA".to_string(),
            remaining_time: 26,
            helper: true,
            turned_on: HashSet::new(),
        };
        aoclib::output(search.bfs(&walker, &self.tunnels, &self.shortcuts))
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    paths: Vec<String>,
}

fn shortcuts(start: &String, tunnels: &HashMap<String, Valve>) -> HashMap<String, usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();

    seen.insert(start);
    queue.push_back((start, 0usize));

    while let Some((node, dist)) = queue.pop_front() {
        let v = tunnels.get(node).unwrap();

        for path in &v.paths {
            if !seen.insert(path) {
                continue;
            }
            let room = tunnels.get(path).unwrap();
            if room.flow_rate > 0 && &room.name != start {
                paths.insert(room.name.clone(), dist + 1);
            }

            queue.push_back((&room.name, dist + 1));
        }
    }

    paths
}

#[derive(Clone)]
struct Walk {
    loc: String,
    remaining_time: usize,
    helper: bool,
    turned_on: HashSet<String>,
}

impl PartialEq for Walk {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc
            && self.helper == other.helper
            && self.remaining_time == other.remaining_time
            && self.turned_on == other.turned_on
    }
}

impl Eq for Walk {}

impl Hash for Walk {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
        self.helper.hash(state);
        self.remaining_time.hash(state);
        let mut vec = self.turned_on.iter().collect::<Vec<&String>>();
        vec.sort();
        for v in vec {
            v.hash(state);
        }
    }
}

#[derive(Default)]
struct Search {
    seen: HashMap<Walk, usize>, // current state -> flow so far
}

impl Search {
    fn bfs(
        &mut self,
        walk: &Walk,
        tunnels: &HashMap<String, Valve>,
        shortcuts: &HashMap<String, HashMap<String, usize>>,
    ) -> usize {
        if let Some(answer) = self.seen.get(walk) {
            return *answer;
        }

        let mut max_flow = if walk.helper {
            // let me walk through the tunnels too, while the elephant is doing
            // the elephant things
            self.bfs(
                &Walk {
                    loc: "AA".to_string(),
                    remaining_time: 26,
                    helper: false,
                    turned_on: walk.turned_on.clone(),
                },
                tunnels,
                shortcuts,
            )
        } else {
            0
        };

        if !walk.turned_on.contains(&walk.loc) && walk.remaining_time > 0 {
            let mut turned_on = walk.turned_on.clone();
            turned_on.insert(walk.loc.clone());
            let flow = tunnels.get(&walk.loc).unwrap().flow_rate * (walk.remaining_time - 1);

            max_flow = max_flow.max(
                self.bfs(
                    &Walk {
                        loc: walk.loc.clone(),
                        remaining_time: walk.remaining_time - 1,
                        helper: walk.helper,
                        turned_on,
                    },
                    tunnels,
                    shortcuts,
                ) + flow,
            );
        }

        let map = shortcuts.get(&walk.loc).unwrap();

        for (dest, cost) in map {
            if *cost < walk.remaining_time {
                max_flow = max_flow.max(self.bfs(
                    &Walk {
                        loc: dest.to_string(),
                        remaining_time: walk.remaining_time - *cost,
                        helper: walk.helper,
                        turned_on: walk.turned_on.clone(),
                    },
                    tunnels,
                    shortcuts,
                ));
            }
        }

        self.seen.insert(walk.clone(), max_flow);
        max_flow
    }
}
