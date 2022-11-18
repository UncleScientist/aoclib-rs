use crate::Runner;

pub struct Aoc2016_18 {
    first_line: String,
}

impl Aoc2016_18 {
    pub fn new() -> Self {
        Self {
            first_line: "".to_string(),
        }
    }
}

impl Runner for Aoc2016_18 {
    fn name(&self) -> (usize, usize) {
        (2016, 18)
    }

    fn parse(&mut self) {
        self.first_line = aoclib::read_lines("input/2016-18.txt")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(count_safe(&self.first_line, 40))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn count_safe(starting_row: &str, mut num: usize) -> usize {
    let mut row = starting_row.to_owned();
    let mut count = 0;

    while num > 0 {
        count += row.chars().filter(|&ch| ch == '.').count();
        num -= 1;
        row = next_row(&row);
    }

    count
}

fn next_row(row: &str) -> String {
    let mut result = "".to_string();
    let row = row.chars().collect::<Vec<char>>();

    result.push(row[1]);

    for window in row.windows(3) {
        if matches!(
            window,
            ['^', '^', '.'] | ['.', '^', '^'] | ['^', '.', '.'] | ['.', '.', '^']
        ) {
            result.push('^');
        } else {
            result.push('.');
        }
    }

    result.push(row[row.len() - 2]);

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_one_row() {
        assert_eq!(".^^^^".to_string(), next_row("..^^."));
    }

    #[test]
    fn calc_next_row() {
        assert_eq!("^^..^".to_string(), next_row(".^^^^"));
    }

    #[test]
    fn test_larger_example() {
        let array = vec![
            ".^^.^.^^^^".to_string(),
            "^^^...^..^".to_string(),
            "^.^^.^.^^.".to_string(),
            "..^^...^^^".to_string(),
            ".^^^^.^^.^".to_string(),
            "^^..^.^^..".to_string(),
            "^^^^..^^^.".to_string(),
            "^..^^^^.^^".to_string(),
            ".^^^..^.^^".to_string(),
            "^^.^^^..^^".to_string(),
        ];
        let mut cur = &array[0];
        for item in array.iter().skip(1) {
            assert_eq!(item, &next_row(cur));
            cur = item;
        }
    }

    #[test]
    fn count_safe_example() {
        assert_eq!(38, count_safe(".^^.^.^^^^", 10));
    }
}
