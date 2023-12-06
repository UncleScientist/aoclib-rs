use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_06 {
    races: Vec<RaceInfo>,
    part2data: RaceInfo,
}

impl Aoc2023_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2023_06 {
    fn name(&self) -> (usize, usize) {
        (2023, 6)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("input/2023-06.txt");
        let _lines = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];

        let times = lines[0]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>();
        let records = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>();

        for (&time, &record) in times.iter().zip(records.iter()) {
            self.races.push(RaceInfo { time, record });
        }

        let part2time = lines[0]
            .split_once(':')
            .unwrap()
            .1
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        let part2record = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        self.part2data = RaceInfo {
            time: part2time,
            record: part2record,
        };
    }

    fn part1(&mut self) -> Vec<String> {
        aoclib::output(self.races.iter().fold(1, |acc, race| acc * race.wins()))
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output(self.part2data.wins())
    }
}
#[derive(Default, Debug)]
struct RaceInfo {
    time: i64,
    record: i64,
}

impl RaceInfo {
    // time t
    // record r
    // button_time -> 0..t
    // movement_time -> (t - button_time)
    // dist = button_time * (t - button_time)
    // dist > r
    //       button_time * (t - button_time) > r
    // solve for button_time:  -button_time^2 + t * button_time - r > 0
    fn wins(&self) -> i64 {
        let a = -1f64;
        let b = self.time as f64;
        let c = (-self.record) as f64;

        // print!("for race {self:?}: ");
        let mut first = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
        let mut second = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();

        if (first * (b - first)) == self.record as f64 {
            first += 1.;
        }
        if (second * (b - second)) == self.record as f64 {
            second -= 1.;
        }

        (second - first) as i64 + 1
    }
}
