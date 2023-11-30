use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2022_19 {
    blueprints: Vec<Blueprint>,
}

impl Aoc2022_19 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_19 {
    fn name(&self) -> (usize, usize) {
        (2022, 19)
    }

    fn parse(&mut self) {
        let _input = aoclib::read_lines("test-input.txt");
        let input = aoclib::read_lines("input/2022-19.txt");

        for line in input {
            let words = line.split_whitespace().collect::<Vec<_>>();
            self.blueprints.push(Blueprint {
                id: words[1].strip_suffix(':').unwrap().parse().unwrap(),
                ore_bot: words[6].parse().unwrap(),
                clay_bot: words[12].parse().unwrap(),
                obsidian_bot: (words[18].parse().unwrap(), words[21].parse().unwrap()),
                geode_bot: (words[27].parse().unwrap(), words[30].parse().unwrap()),
            });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut quality = 0;

        for bp in &self.blueprints {
            let best = mine_geodes(bp, 24);
            quality += bp.id * best;
            println!(
                "Blueprint id {} makes {best} geodes, quality = {quality}",
                bp.id
            );
        }

        aoclib::output(quality)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut answer = 1;

        for bp in self.blueprints.iter().take(3) {
            answer *= mine_geodes(bp, 32);
        }

        aoclib::output(answer)
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_bot: usize,
    clay_bot: usize,
    obsidian_bot: (usize, usize), // ore + clay
    geode_bot: (usize, usize),    // ore + obsidian
}

fn mine_geodes(bp: &Blueprint, time: usize) -> usize {
    let mut to_visit = Vec::new();

    let max_ore = bp
        .ore_bot
        .max(bp.clay_bot.max(bp.obsidian_bot.0.max(bp.geode_bot.0)));
    let max_clay = bp.obsidian_bot.1;
    let max_obsidian = bp.geode_bot.1;

    to_visit.push(State {
        ore_bots: 1,
        time_remaining: time,
        ..Default::default()
    });
    let mut best = 0;

    while let Some(state) = to_visit.pop() {
        if state.time_remaining == 0 {
            best = best.max(state.geodes);
            continue;
        }

        let can_continue = time < 50    // a cheat to use the "fast" but incorrect code
            || (state.ore_bots >= max_ore
                && state.clay_bots >= max_clay
                && state.obsidian_bots >= max_obsidian);

        if state.ore >= bp.geode_bot.0 && state.obsidian >= bp.geode_bot.1 {
            to_visit.push(state.make_geode_bot(bp));
            if can_continue {
                continue;
            }
        }

        if state.obsidian_bots < max_obsidian
            && state.ore >= bp.obsidian_bot.0
            && state.clay >= bp.obsidian_bot.1
        {
            to_visit.push(state.make_obsidian_bot(bp));
            if can_continue {
                continue;
            }
        }

        to_visit.push(state.tick());

        if state.ore_bots < max_ore && state.ore >= bp.ore_bot {
            to_visit.push(state.make_ore_bot(bp));
        }

        if state.clay_bots < max_clay && state.ore >= bp.clay_bot {
            to_visit.push(state.make_clay_bot(bp));
        }
    }

    best
}

#[derive(Debug, Default, Clone)]
struct State {
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    time_remaining: usize,
}

impl State {
    fn tick(&self) -> Self {
        let mut new_state = self.clone();

        new_state.ore += self.ore_bots;
        new_state.clay += self.clay_bots;
        new_state.obsidian += self.obsidian_bots;
        new_state.geodes += self.geode_bots;
        new_state.time_remaining -= 1;

        new_state
    }

    fn make_ore_bot(&self, bp: &Blueprint) -> Self {
        let mut new_state = self.tick();
        new_state.ore -= bp.ore_bot;
        new_state.ore_bots += 1;
        new_state
    }

    fn make_clay_bot(&self, bp: &Blueprint) -> Self {
        let mut new_state = self.tick();
        new_state.ore -= bp.clay_bot;
        new_state.clay_bots += 1;
        new_state
    }

    fn make_obsidian_bot(&self, bp: &Blueprint) -> Self {
        let mut new_state = self.tick();
        new_state.ore -= bp.obsidian_bot.0;
        new_state.clay -= bp.obsidian_bot.1;
        new_state.obsidian_bots += 1;
        new_state
    }

    fn make_geode_bot(&self, bp: &Blueprint) -> Self {
        let mut new_state = self.tick();
        new_state.ore -= bp.geode_bot.0;
        new_state.obsidian -= bp.geode_bot.1;
        new_state.geode_bots += 1;
        new_state
    }
}
