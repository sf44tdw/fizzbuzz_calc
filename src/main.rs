use std::io;
use std::num::ParseIntError;

fn main() {
    const FIZZ :&str ="Fizz";
    const BUZZ :&str ="Buzz";

    let _exit :String = "exit".to_string(); 

loop{
    println!("Please input number or {}(break loop).",_exit);

    let mut _line: String = String::new();
    io::stdin().read_line(&mut _line).expect("Failed to read line");

    //改行を除去。
    let _line_trim: &str = _line.trim();
    
    if _exit.eq(_line_trim) {
        println!("Break loop.");
        break;
        }

    let r_number: Result<i64, ParseIntError> = _line_trim.parse::<i64>();

    if r_number.is_ok(){
        let number : i64 = r_number.unwrap();
        println!("Number is {}.",number);

        let mut ret: String = String::new();
        if number >= 1{
            if number % 3 == 0{
                ret+=FIZZ;
            }
            if number % 5 == 0{
                ret=ret+BUZZ;
            }
        }else{
            ret="Less than 1.".to_string();
        }
        
        println!("{}",ret);
    }else{
        println! ("{} is not u64 number.",_line_trim);
    }
}
}
