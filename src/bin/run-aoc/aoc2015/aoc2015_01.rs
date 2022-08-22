use aoclib_rs::read_to_chars;

pub fn aoc2015_01() {
    let data = read_to_chars("input/2015-01.txt");
    println!(
        "2015 day 1 part 1 = {}",
        data.iter()
            .map(|x| match x {
                '(' => 1,
                ')' => -1,
                _ => panic!("invalid char in input"),
            })
            .sum::<i32>()
    );
}
