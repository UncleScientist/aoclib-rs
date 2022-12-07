use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::Runner;

#[derive(Default)]
pub struct Aoc2022_07 {
    root: Rc<Dir>,
}

impl Aoc2022_07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 7)
    }

    fn parse(&mut self) {
        let mut cwd = Rc::clone(&self.root);
        for line in aoclib::read_lines("input/2022-07.txt") {
            let words = line.split(' ').collect::<Vec<&str>>();
            match (words[0], words[1]) {
                ("$", "ls") => {}
                ("$", "cd") => match words[2] {
                    "/" => cwd = Rc::clone(&self.root),
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                        let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                },
                ("dir", dirname) => {
                    cwd.subdir.borrow_mut().insert(
                        dirname.to_string(),
                        Rc::new(Dir {
                            _name: dirname.to_string(),
                            size: RefCell::new(0),
                            parent: Some(Rc::clone(&cwd)),
                            subdir: RefCell::new(HashMap::new()),
                        }),
                    );
                }
                (size, _name) => {
                    *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut total = 0;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
        }

        crate::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let total_size = self.root.get_size();
        let free_space = 70000000 - total_size;
        let space_needed = 30000000 - free_space;

        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut best = usize::MAX;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size >= space_needed {
                best = best.min(size);
            }
        }
        crate::output(best)
    }
}

#[derive(Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}
