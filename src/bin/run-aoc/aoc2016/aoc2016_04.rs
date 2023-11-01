use std::collections::HashMap;

use crate::Runner;

pub struct Aoc2016_04 {
    room: Vec<Room>,
}

impl Aoc2016_04 {
    pub fn new() -> Self {
        Self { room: Vec::new() }
    }
}

impl Runner for Aoc2016_04 {
    fn name(&self) -> (usize, usize) {
        (2016, 4)
    }

    fn parse(&mut self) {
        let _test_data = [
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];
        for v in &aoclib::read_lines("input/2016-04.txt") {
            self.room.push(Room::new(v));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for r in &self.room {
            if r.is_real() {
                total += r.sector;
            }
        }
        crate::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        for r in &self.room {
            if r.is_real() && r.decrypt() == "northpole object storage" {
                return crate::output(r.sector);
            }
        }
        crate::output("No Sector Found")
    }
}

struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn new(room: &str) -> Self {
        let (room_str, cksum) = room.split_once('[').unwrap();
        let idx = room_str.rfind('-').unwrap();
        let (name, sector_str) = room_str.split_at(idx);
        let sector = -sector_str.parse::<i32>().unwrap();
        let mut checksum = cksum.to_string();
        checksum.pop();

        Self {
            name: name.to_string(),
            sector,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let mut hm = HashMap::new();
        for c in self.name.chars().filter(|x| x.is_alphabetic()) {
            *hm.entry(c).or_insert(0) += 1;
        }

        let mut list: Vec<(&char, &i32)> = hm.iter().collect();
        list.sort_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        5 == list
            .iter()
            .take(5)
            .fold(0, |a, (&k, _)| a + (self.checksum.contains(k) as i32))
    }

    fn decrypt(&self) -> String {
        let mut result = "".to_string();

        let m = (self.sector % 26) as u8;
        for c in self.name.chars() {
            if c == '-' {
                result.push(' ');
            } else {
                result.push((b'a' + ((c as u8 - b'a' + m) % 26)) as char);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decrypt_name() {
        let room = Room::new("qzmt-zixmtkozy-ivhz-343[asdfg]");
        assert_eq!(room.decrypt(), "very encrypted name".to_string());
    }
}
