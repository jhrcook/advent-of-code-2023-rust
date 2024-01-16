use num::integer;

// Algorithm source: https://www.geeksforgeeks.org/lcm-of-given-array-elements/
pub fn lcm(a: Vec<u64>) -> u64 {
    let mut lcm = a[0];
    for i in a.iter() {
        lcm = integer::div_floor(lcm * *i, integer::gcd(lcm, *i));
    }
    lcm
}
