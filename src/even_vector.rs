fn only_even(v: Vec<i32>) -> Vec<i32> {
    let mut even_vector: Vec<i32> = Vec::new(); 
    for i in v {
        if i % 2 == 0 {
            even_vector.push(i);
        } else {
            continue;
        }
    }
    even_vector
}

fn main() {
    let a = only_even(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    for i in a {
        println!("{}", i);
    }
}