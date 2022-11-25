fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6];
    let b = &mut a[1..3];

    for i in 0..b.len() {
        b[i] = b[i] * 2;
    }

    assert_eq!(a, &[1, 4, 6, 4, 5, 6]);
    println!("{:?}", a);
}