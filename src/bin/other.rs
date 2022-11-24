fn main() {
    let a = 0..10;

    for (i, j) in a.enumerate().rev() {
        println!("{} {}", i, j);
    }
}