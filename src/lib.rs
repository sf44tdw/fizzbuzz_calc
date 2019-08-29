extern crate failure;

use failure::*;

pub fn fizz(x: i64) -> String {
    return is_zero_modulo(x, 3, "Fizz".to_string()).unwrap();
}

pub fn buzz(x: i64) -> String {
    return is_zero_modulo(x, 5, "Buzz".to_string()).unwrap();
}

fn is_zero_modulo(a: i64, b: i64, return_if_mod_zero: String) -> Result<String, failure::Error> {
    if b == 0 {
        return Err(format_err!("a:{} b:{} Can not execute a mod b.", a, b));
    }
    let answer_mod: i64 = a % b;
    if answer_mod == 0 {
        Ok(return_if_mod_zero)
    } else {
        Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "Can not execute a mod b")]
    fn zero_devide_panic() {
        #[warn(unused_variables)]
        let _string_sample = is_zero_modulo(6, 0, "panic".to_string()).unwrap();
    }
}
