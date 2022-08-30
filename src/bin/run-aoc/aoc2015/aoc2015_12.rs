use crate::Runner;
use aoclib::read_lines;
use serde_json::Value;

pub struct Aoc2015_12 {
    value: Option<Value>,
}

impl Aoc2015_12 {
    pub fn new() -> Self {
        Self { value: None }
    }
}

impl Runner for Aoc2015_12 {
    fn parse(&mut self) {
        self.value = Some(
            serde_json::from_str(&read_lines("input/2015-12.txt")[0])
                .expect("corrupted input file"),
        );
    }

    fn name(&self) -> (usize, usize) {
        (2015, 12)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(sum_json(&self.value.as_ref().unwrap()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(sum_redless(&self.value.as_ref().unwrap()).unwrap())
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

fn sum_redless(v: &Value) -> Option<i64> {
    match v {
        Value::Null | Value::Bool(_) => Some(0),
        Value::String(s) => {
            if s == "red" {
                None
            } else {
                Some(0)
            }
        }
        Value::Number(n) => Some(n.as_i64().unwrap()),
        Value::Array(a) => Some(a.iter().map(|entry| sum_redless(entry).unwrap_or(0)).sum()),
        Value::Object(o) => {
            let (somes, nones): (Vec<_>, Vec<_>) = o
                .values()
                .map(|entry| sum_redless(entry))
                .partition(Option::is_some);

            if nones.is_empty() {
                Some(somes.iter().map(|entry| entry.unwrap()).sum())
            } else {
                Some(0)
            }
        }
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
    fn redless_array() {
        let v = serde_json::from_str("[1,2,3]").expect("failed to parse json");
        assert_eq!(6, sum_redless(&v).unwrap());
    }

    #[test]
    fn array_with_red() {
        let v = serde_json::from_str(r#"[1,"red",5]"#).expect("failed to parse json");
        assert_eq!(6, sum_redless(&v).unwrap());
    }

    #[test]
    fn redless_object_zero() {
        let v = serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)
            .expect("failed to parse json");
        assert_eq!(0, sum_redless(&v).unwrap());
    }

    #[test]
    fn redless_object() {
        let v = serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).expect("failed to parse json");
        assert_eq!(4, sum_redless(&v).unwrap());
    }

    #[test]
    fn can_sum_array() {
        let v = serde_json::from_str("[1,2,3]").expect("failed to parse json");
        assert_eq!(6, sum_json(&v));
    }
}
