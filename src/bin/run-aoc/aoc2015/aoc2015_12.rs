use crate::Runner;
use aoclib::read_lines;
use serde_json::Value;

pub struct Aoc2015_12 {
    lines: Vec<String>,
}

impl Aoc2015_12 {
    pub fn new() -> Self {
        Self {
            lines: read_lines("input/2015-12.txt"),
        }
    }
}

impl Runner for Aoc2015_12 {
    fn name(&self) -> (usize, usize) {
        (2015, 12)
    }

    fn part1(&mut self) -> Vec<String> {
        let json = serde_json::from_str(&self.lines[0]).expect("corrupted input file");
        crate::output(sum_json(&json))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("unsolved")
    }
}

fn sum_json(v: &Value) -> i64 {
    match v {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|entry| sum_json(entry)).sum(),
        Value::Object(o) => o.values().map(|entry| sum_json(entry)).sum(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_array() {
        let v = serde_json::from_str(r#"[]"#).expect("failed to parse json");
        assert_eq!(0, sum_json(&v));
    }

    #[test]
    fn nested_array() {
        let v = serde_json::from_str(r#"[[[3]]]"#).expect("failed to parse json");
        assert_eq!(3, sum_json(&v));
    }

    #[test]
    fn can_sum_object() {
        let v = serde_json::from_str(r#"{"a":2,"b":4}"#).expect("failed to parse json");
        assert_eq!(6, sum_json(&v));
    }

    #[test]
    fn can_sum_array() {
        let v = serde_json::from_str("[1, 2, 3]").expect("failed to parse json");
        assert_eq!(6, sum_json(&v));
    }
}
