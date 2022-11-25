fn main() {
    let a = vec![1, 2, 3, 4, 5, 6];
    let reps = 3;
    let a_broadcasted = {
        let mut _a = Vec::<&i32>::with_capacity(reps * a.len());
        let a_ptr = a.as_ptr();
        for j in 0..a.len() {
            _a.push(unsafe { a_ptr.add(j).as_ref().unwrap() })
        }
        _a
    };

    let mut out = vec![0; a_broadcasted.len()];

    for i in 0..a_broadcasted.len() {
        out[i] = *a_broadcasted[i] * 2;
    }

    // assert_eq!(a, &[1, 4, 6, 4, 5, 6]);
    println!("{:?}", out);
}