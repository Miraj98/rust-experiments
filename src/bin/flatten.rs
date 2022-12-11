fn main() {
    let a = [[1, 2], [3, 4]];
    println!("{:?}", a.iter().flatten().collect::<Vec<_>>().as_slice());
}