fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let a: &str = "hello";
    println!("{}", reverse(a));
}