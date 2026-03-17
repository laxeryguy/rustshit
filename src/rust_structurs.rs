struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {width: 5, height: 5};
    println!("{}", r.area());
}