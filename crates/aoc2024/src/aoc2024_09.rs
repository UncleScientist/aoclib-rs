use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2024_09 {
    disk: Vec<usize>,
}

impl Aoc2024_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2024_09 {
    fn name(&self) -> (usize, usize) {
        (2024, 9)
    }

    fn parse(&mut self) {
        let line = aoclib::read_single_line("input/2024-09.txt");
        self.disk = line
            .iter()
            .map(|ch| ((*ch as u8) - b'0') as usize)
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut fileno = 0_i64;
        let mut disk = Vec::new();
        let mut iter = self.disk.iter();
        while let Some(len) = iter.next() {
            disk.extend(vec![fileno; *len]);
            fileno += 1;
            if let Some(gap) = iter.next() {
                disk.extend(vec![-1; *gap]);
            }
        }

        while let Some(free_space) = disk.iter().position(|val| *val == -1) {
            while let Some(val) = disk.pop() {
                if val == -1 {
                    continue;
                }
                if free_space >= disk.len() {
                    disk.push(val);
                    break;
                }
                disk[free_space] = val;
                break;
            }
        }

        aoclib::output(
            disk.iter()
                .enumerate()
                .map(|(idx, val)| idx as i64 * val)
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut fileno = 0;
        let mut iter = self.disk.iter();

        let mut file_list = Vec::new();
        let mut gap_list = Vec::new();

        let mut pos = 0;
        while let Some(len) = iter.next() {
            file_list.push((pos, len, fileno));
            fileno += 1;
            pos += len;
            if let Some(gap) = iter.next() {
                gap_list.push((pos, *gap));
                pos += gap;
            }
        }

        let mut updated_files = Vec::new();

        while let Some(mut file) = file_list.pop() {
            let mut moved = false;
            for i in 0..gap_list.len() {
                if gap_list[i].0 >= file.0 {
                    updated_files.push(file);
                    moved = true;
                    break;
                }

                if gap_list[i].1 >= *file.1 {
                    file.0 = gap_list[i].0;
                    updated_files.push(file);
                    moved = true;

                    if gap_list[i].1 > *file.1 {
                        gap_list[i].0 += *file.1;
                        gap_list[i].1 -= *file.1;
                    } else {
                        gap_list.remove(i);
                    }
                    break;
                }
            }
            if !moved {
                updated_files.push(file);
            }
        }
        updated_files.sort_by_key(|file| file.0);

        let total = updated_files
            .iter()
            .map(|(pos, len, fileno)| {
                (*pos..*pos + **len)
                    .map(|index| index * fileno)
                    .sum::<usize>()
            })
            .sum::<usize>();
        aoclib::output(total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() {
        let mut aoc = Aoc2024_09 {
            disk: "2333133121414131402"
                .chars()
                .map(|ch| ((ch as u8) - b'0') as usize)
                .collect(),
        };
        let expected = vec!["2858".to_string()];
        assert_eq!(expected, aoc.part2());
    }
}
