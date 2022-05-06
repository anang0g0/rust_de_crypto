fn take(v: Vec<i32>) -> Vec<i32> {
    return v;
}

fn main() {
    let mut v = vec![1, 2, 3];

    v = take(v);

    println!("v[0] is: {}", v[0]);
}
