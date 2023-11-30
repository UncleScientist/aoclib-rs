use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2017_10 {
    list: Vec<u8>,
    nums: Vec<u8>,
}

impl Aoc2017_10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2017_10 {
    fn name(&self) -> (usize, usize) {
        (2017, 10)
    }

    fn parse(&mut self) {
        let data = aoclib::read_lines("input/2017-10.txt");
        self.list = data[0]
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        self.nums = data[0][0..data[0].len()].chars().map(|x| x as u8).collect();
        self.nums.extend([17, 31, 73, 47, 23]);
    }

    fn part1(&mut self) -> Vec<String> {
        let mut list = (0..=255).collect::<Vec<i32>>();
        rounds(&mut list, 1, &self.list);
        aoclib::output(list[0] * list[1])
    }

    fn part2(&mut self) -> Vec<String> {
        let mut list = (0..=255).collect::<Vec<i32>>();

        rounds(&mut list, 64, &self.nums);

        let mut answer = "".to_string();
        for chunk in list.chunks(16) {
            answer.push_str(&format!("{:02x}", chunk.iter().fold(0, |a, b| a ^ *b)));
        }
        aoclib::output(answer)
    }
}

fn rounds(list: &mut Vec<i32>, rounds: usize, input: &[u8]) {
    let mut curpos = 0;
    let mut skip = 0;

    for _ in 0..rounds {
        for i in input.iter() {
            for p in 0..(*i as usize) / 2 {
                let a = (curpos + p) % list.len();
                let b = ((*i as usize - p) + curpos - 1) % list.len();
                list.swap(a, b);
            }

            curpos = (curpos + *i as usize + skip) % list.len();
            skip += 1;
        }
    }
}
