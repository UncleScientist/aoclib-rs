use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_06 {
    races: Vec<RaceInfo>,
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
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>();
        let records = lines[1]
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect::<Vec<i64>>();

        for (&time, &record) in times.iter().zip(records.iter()) {
            self.races.push(RaceInfo { time, record });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        // time t
        // record r
        // button_time 0..t
        // movement_time (t - button_time)
        // dist = button_time * (t - button_time)
        // dist > r
        //   button_time * (t - button_time) > r
        // solve for button_time:  -button_time^2 + t * button_time - r > 0

        let mut ans = 1;
        for race in &self.races {
            let a = -1f64;
            let b = race.time as f64;
            let c = (-race.record) as f64;

            // print!("for race {race:?}: ");
            let mut first = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
            let mut second = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();

            if (first * (b - first)) == race.record as f64 {
                first += 1.;
            }
            if (second * (b - second)) == race.record as f64 {
                second -= 1.;
            }

            // println!("{first}..{second}");

            ans *= (second - first) as i64 + 1;
        }
        aoclib::output(ans)
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct RaceInfo {
    time: i64,
    record: i64,
}
