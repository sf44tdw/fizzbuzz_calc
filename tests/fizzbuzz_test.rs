extern crate fizzbuzz_calc;

use fizzbuzz_calc::buzz;
use fizzbuzz_calc::fizz;

#[test]
fn fizz_works_3() {
    let x: u64 = 3;
    let fz: String = fizz(x);
    assert_eq!(fz, "Fizz");
}

#[test]
fn fizz_works_15() {
    let x: u64 = 15;
    let fz: String = fizz(x);
    assert_eq!(fz, "Fizz");
}

#[test]
fn buzz_works_5() {
    let x: u64 = 5;
    let bz: String = buzz(x);
    assert_eq!(bz, "Buzz");
}

#[test]
fn buzz_works_15() {
    let x: u64 = 15;
    let bz: String = buzz(x);
    assert_eq!(bz, "Buzz");
}
