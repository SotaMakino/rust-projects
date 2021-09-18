fn main() {
    let vec = vec![1, 2, 3];
    let v: Vec<i32> = vec.iter().map(|x| x + 1).collect();

    println!("{:?}", v);
}
