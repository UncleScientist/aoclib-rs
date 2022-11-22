use crate::Runner;

pub struct Aoc2016_22 {
    nodes: Vec<GridNode>,
}

impl Aoc2016_22 {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

struct GridNode {
    loc: (usize, usize),
    used: usize,
    avail: usize,
}

/*
root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     94T   67T    27T   71%
/dev/grid/node-x0-y1     89T   67T    22T   75%
/dev/grid/node-x0-y2     93T   69T    24T   74%
/dev/grid/node-x0-y3     91T   66T    25T   72%
/dev/grid/node-x0-y4     85T   69T    16T   81%
/dev/grid/node-x0-y5     89T   64T    25T   71%
/dev/grid/node-x0-y6     88T   70T    18T   79%
/dev/grid/node-x0-y7     89T   65T    24T   73%
*/

impl Runner for Aoc2016_22 {
    fn name(&self) -> (usize, usize) {
        (2016, 22)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2016-22.txt");

        for l in lines.iter().skip(2) {
            let col = l.split_whitespace().collect::<Vec<&str>>();
            let loc = col[0].split('-').skip(1).collect::<Vec<&str>>();
            let loc = (
                loc[0][1..].parse::<usize>().unwrap(),
                loc[1][1..].parse::<usize>().unwrap(),
            );
            let used = col[2][0..col[2].len() - 1].parse::<usize>().unwrap();
            let avail = col[3][0..col[3].len() - 1].parse::<usize>().unwrap();
            self.nodes.push(GridNode { loc, used, avail });
        }
        println!("{}", self.nodes.len());
    }

    fn part1(&mut self) -> Vec<String> {
        let count = self.nodes.iter().filter(|a| a.used != 0).fold(0, |sum, a| {
            sum + self
                .nodes
                .iter()
                .filter(|b| a.loc != b.loc && a.used <= b.avail)
                .count()
        });
        crate::output(count)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}
