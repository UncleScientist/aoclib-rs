use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2019_08 {
    img_data: Vec<char>,
}

impl Aoc2019_08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2019_08 {
    fn name(&self) -> (usize, usize) {
        (2019, 8)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2019-08.txt");
        self.img_data = lines[0].chars().collect::<Vec<_>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let answer = self
            .img_data
            .chunks(25 * 6)
            .map(|chunk| {
                chunk.iter().fold([0, 0, 0], |mut counts, ch| {
                    counts[(*ch as u8 - b'0') as usize] += 1;
                    counts
                })
            })
            .min_by_key(|count| count[0])
            .unwrap();
        aoclib::output(answer[1] * answer[2])
    }

    fn part2(&mut self) -> Vec<String> {
        let mut image: Vec<char> = vec!['2'; 25 * 6];

        self.img_data.chunks(25 * 6).for_each(|chunk| {
            for row in 0..6 {
                for col in 0..25 {
                    if image[row * 25 + col] == '2' {
                        image[row * 25 + col] = chunk[row * 25 + col];
                    }
                }
            }
        });

        image
            .chunks(25)
            .map(|slice| {
                slice
                    .iter()
                    .map(|ch| match ch {
                        '1' => '#',
                        '0' => '.',
                        _ => panic!("oops"),
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
    }
}
