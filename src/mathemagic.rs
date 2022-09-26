pub fn power_mod<T>(start: T, mut base: T, mut exp: T, modulus: T) -> T
where
    T: std::ops::Rem<Output = T>
        + std::ops::RemAssign
        + TryFrom<i32>
        + Copy
        + std::cmp::PartialOrd
        + std::ops::ShrAssign
        + std::ops::Mul<Output = T>,
    <T as std::convert::TryFrom<i32>>::Error: std::fmt::Debug,
{
    let mut result = start % modulus;
    base %= modulus;

    let zero: T = 0_i32.try_into().unwrap();
    let one: T = 1_i32.try_into().unwrap();
    let two: T = 2_i32.try_into().unwrap();

    while exp > zero {
        if exp % two == one {
            result = (result * base) % modulus;
        }

        exp >>= one;
        base = (base * base) % modulus;
    }

    result
}
