pub fn knot_hash(input: &str) -> u128 {
    let mut list = (0..=255).collect::<Vec<u8>>();

    let mut curpos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for c in input.chars().map(|c| c as u8).chain([17u8, 31, 73, 47, 23]) {
            for p in 0..(c as usize) / 2 {
                let a = (curpos + p) % list.len();
                let b = ((c as usize - p) + curpos - 1) % list.len();
                list.swap(a, b);
            }

            curpos = (curpos + c as usize + skip) % list.len();
            skip += 1;
        }
    }

    list.chunks(16).fold(0u128, |a, b| {
        a << 8 | (b.iter().fold(0u8, |c, d| c ^ *d)) as u128
    })
}
