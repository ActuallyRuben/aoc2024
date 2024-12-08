mod refgrid;
mod sortedvec;
mod permutations;

use std::cmp::Ordering;
pub use refgrid::Grid;
pub use permutations::Permutable;

pub fn gcd(mut a: isize, mut b: isize) -> isize {
    loop {
        if b == 0 {
            return a;
        }
        (a, b) = (b, a.rem_euclid(b))
    }
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(10, 5), 5);
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(81, 54), 27);
}