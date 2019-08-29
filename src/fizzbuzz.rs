pub fn fizz(x: i64) -> String {
    return is_zero_modulo(x, 3, "Fizz".to_string());
}

pub fn buzz(x: i64) -> String {
    return is_zero_modulo(x, 5, "Buzz".to_string());
}

fn is_zero_modulo(x: i64, y: i64, ret: String) -> String {
    if x % y == 0 {
        return ret;
    } else {
        return "".to_string();
    }
}

#[test]
fn fizz_works_3() {
    let x: i64 = 3;
    let fz: String = fizz(x);
    assert_eq!(fz, "Fizz");
}

#[test]
fn fizz_works_15() {
    let x: i64 = 15;
    let fz: String = fizz(x);
    assert_eq!(fz, "Fizz");
}

#[test]
fn buzz_works_5() {
    let x: i64 = 5;
    let bz: String = buzz(x);
    assert_eq!(bz, "Buzz");
}

#[test]
fn buzz_works_15() {
    let x: i64 = 15;
    let bz: String = buzz(x);
    assert_eq!(bz, "Buzz");
}
