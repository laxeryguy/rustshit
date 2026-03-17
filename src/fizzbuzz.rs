use std::io::{self, Read};

//  Fizz3 Buzz5 FizzBuzz35

fn main() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("gg");
    
    let s: i32 = s.trim().parse().expect("gg");

    if s % 3 == 0 && s % 5 == 0 {
        println!("FizzBuzz");
    } else if s % 3 == 0 {
        println!("Fizz");
    } else if s % 5 == 0 {
        println!("Buzz");
    } else {
        println!("Unlucky");
    }
}
