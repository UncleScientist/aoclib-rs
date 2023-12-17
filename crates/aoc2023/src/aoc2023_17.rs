use aoclib::{astar_search, Nodes, Runner, Searcher};

#[derive(Default)]
pub struct Aoc2023_17 {
    city: Vec<Vec<u8>>,
    width: isize,
    height: isize,
}

impl Aoc2023_17 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_17 {
    fn name(&self) -> (usize, usize) {
        (2023, 17)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-17.txt");
        // let lines = aoclib::read_lines("test-input");

        for line in lines {
            self.city
                .push(line.chars().map(|ch| ((ch as u8) - b'0')).collect());
        }

        self.height = self.city.len() as isize;
        self.width = self.city[0].len() as isize;
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(
            astar_search(
                &State {
                    pos: (0, 0),
                    dir: (0, 1),
                    count: 0,
                },
                |state| ((self.height - state.pos.0) + (self.width - state.pos.1)) as usize,
                self,
            )
            .unwrap()
            .1,
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

impl Nodes for Aoc2023_17 {
    fn get_value(&self, row: usize, col: usize) -> usize {
        self.city[row][col] as usize
    }

    fn get_width(&self) -> usize {
        self.width as usize
    }

    fn get_height(&self) -> usize {
        self.height as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (isize, isize),
    dir: (isize, isize),
    count: usize,
}

impl Searcher for State {
    fn moves<N: Nodes>(&self, nodes: &N) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut succ = Vec::new();
        let width = nodes.get_width() as isize;
        let height = nodes.get_height() as isize;

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        for dir in dirs {
            // mustn't go backwards
            if dir == (-self.dir.0, -self.dir.1) {
                continue;
            }

            // mustn't go off the map
            let pos = (self.pos.0 + dir.0, self.pos.1 + dir.1);
            if pos.0 < 0 || pos.0 >= height || pos.1 < 0 || pos.1 >= width {
                continue;
            }

            if self.dir == dir && self.count < 3 {
                succ.push(State {
                    pos,
                    dir,
                    count: self.count + 1,
                });
            } else if self.dir != dir {
                succ.push(State { pos, dir, count: 1 });
            }
        }

        // println!("From {self:?}: {succ:?}");

        succ
    }

    fn is_win_state<N: Nodes>(&self, nodes: &N) -> bool {
        let end = (
            nodes.get_height() as isize - 1,
            nodes.get_width() as isize - 1,
        );
        self.pos == end
    }

    fn cost<N: Nodes>(&self, nodes: &N) -> usize {
        nodes.get_value(self.pos.0 as usize, self.pos.1 as usize)
    }
}
