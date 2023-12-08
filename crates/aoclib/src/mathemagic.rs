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

pub fn gcd<T>(x: T, y: T) -> T
where
    T: Copy
        + std::convert::From<u8>
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::convert::From<u8>,
{
    let zero: T = 0_u8.try_into().unwrap();
    if y == zero {
        x
    } else {
        gcd(y, x % y)
    }
}

pub fn gcd_of<T>(list: &[T]) -> T
where
    T: Copy
        + std::convert::From<u8>
        + std::cmp::PartialEq<u8>
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>,
{
    list.iter().copied().reduce(gcd).unwrap()
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Copy
        + std::convert::From<u8>
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    x * y / gcd(x, y)
}

pub fn lcm_of<T>(list: &[T]) -> T
where
    T: Copy
        + std::convert::From<u8>
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    list.iter().copied().reduce(lcm).unwrap()
}
