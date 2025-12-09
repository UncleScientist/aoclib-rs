#[derive(Debug)]
struct Node {
    parent: usize,
    size: usize,
    rank: usize,
}

#[derive(Debug)]
enum By {
    Rank,
    Size,
}

#[derive(Debug)]
pub struct DisjointSet {
    nodes: Vec<Node>,
    by: By,
}

impl DisjointSet {
    pub fn init_by_size(count: usize) -> Self {
        Self {
            by: By::Size,
            nodes: (0..count)
                .map(|id| Node {
                    parent: id,
                    size: 1,
                    rank: 0,
                })
                .collect(),
        }
    }

    pub fn init_by_rank(count: usize) -> Self {
        Self {
            by: By::Rank,
            nodes: (0..count)
                .map(|id| Node {
                    parent: id,
                    size: 1,
                    rank: 0,
                })
                .collect(),
        }
    }

    pub fn find(&mut self, id: usize) -> usize {
        if self.nodes[id].parent != id {
            self.nodes[id].parent = self.find(self.nodes[id].parent);
            return self.nodes[id].parent;
        }
        id
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return false;
        }

        let (x, y) = if self.smaller(x, y) { (y, x) } else { (x, y) };

        self.nodes[y].parent = x;
        self.nodes[x].size += self.nodes[y].size;
        if self.nodes[x].rank == self.nodes[y].rank {
            self.nodes[x].rank += 1;
        }

        true
    }

    pub fn get_sizes(&self) -> Option<Vec<usize>> {
        match self.by {
            By::Rank => None,
            By::Size => Some(self.nodes.iter().map(|node| node.size).collect()),
        }
    }

    fn smaller(&self, x: usize, y: usize) -> bool {
        match self.by {
            By::Rank => self.nodes[x].rank < self.nodes[y].rank,
            By::Size => self.nodes[x].size < self.nodes[y].size,
        }
    }
}

