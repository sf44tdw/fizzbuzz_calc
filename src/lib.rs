extern crate failure;

use failure::*;
/// 渡された数値が3の倍数であれば"Fizz"を返す。
/// それ以外の場合は空文字列を返す。
///
/// # Failures
/// x = 0の場合、エラーになる。
///
/// # Examples
/// let c = fizz(0);
/// let c = fizz(3);
///
pub fn fizz(x: u64) -> String {
    return is_zero_modulo_without_zero_dividend(x, 3, "Fizz".to_string(), "".to_string()).unwrap();
}

/// 渡された数値が5の倍数であれば"buzz"を返す。
/// それ以外の場合は空文字列を返す。
///
/// # Failures
/// x = 0の場合、エラーになる。
///
/// # Examples
/// let c = buzz(0);
/// let c = buzz(5);
///
pub fn buzz(x: u64) -> String {
    return is_zero_modulo_without_zero_dividend(x, 5, "Buzz".to_string(), "".to_string()).unwrap();
}

/// a mod b の結果次第で渡されたメッセージのどちらかを返す。
///
/// # Failures
/// a = 0の時はエラーになる。
///
/// # Examples
/// let c = is_zero_modulo(10, 2, "zero".to_string(),"not zero".to_string()).unwrap();
///
fn is_zero_modulo_without_zero_dividend(
    a: u64,
    b: u64,
    return_if_mod_is_zero_message: String,
    return_if_mod_is_not_zero_message: String,
) -> Result<String, failure::Error> {
    if a == 0 {
        return Err(format_err!("a:{} b:{} FizzBuzz start 1.", a, b));
    }
    return is_zero_modulo(
        a,
        b,
        return_if_mod_is_zero_message,
        return_if_mod_is_not_zero_message,
    );
}

/// a mod b の結果次第で渡されたメッセージのどちらかを返す。
///
/// # Failures
/// b = 0の時はエラーになる。
///
/// # Examples
/// let c = is_zero_modulo(10, 2, "zero".to_string(),"not zero".to_string()).unwrap();
///
fn is_zero_modulo(
    a: u64,
    b: u64,
    return_if_mod_is_zero_message: String,
    return_if_mod_is_not_zero_message: String,
) -> Result<String, failure::Error> {
    if b == 0 {
        return Err(format_err!("a:{} b:{} Can not execute a mod b.", a, b));
    }
    let answer_mod: u64 = a % b;
    if answer_mod == 0 {
        Ok(return_if_mod_is_zero_message)
    } else {
        Ok(return_if_mod_is_not_zero_message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "FizzBuzz start 1")]
    fn panic_dividend_is_zero() {
        #[warn(unused_variables)]
        let _string_sample =
            is_zero_modulo_without_zero_dividend(0, 1, "panic".to_string(), "panic".to_string())
                .unwrap();
    }

    #[test]
    #[should_panic(expected = "Can not execute a mod b")]
    fn panic_division_by_zero() {
        #[warn(unused_variables)]
        let _string_sample =
            is_zero_modulo(6, 0, "panic".to_string(), "panic".to_string()).unwrap();
    }
}
