pub fn power_mod(start: u64, mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = start % modulus;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }

        exp >>= 1;
        base = (base * base) % modulus;
    }

    result
}
