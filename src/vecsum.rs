fn sum(v: Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    for i in v {
        s += i;
    }
    s
}

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", sum(a));
}