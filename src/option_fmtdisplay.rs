fn divide(nf: i32, ns: i32) -> Option<f64> {
    if ns != 0 {
        Some(nf as f64 / ns as f64)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", divide(10, 5));
}