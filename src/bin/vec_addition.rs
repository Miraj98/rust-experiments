use std::{time::Instant, ptr::NonNull, mem::ManuallyDrop};

use rand::{distributions::Uniform, Rng};

fn main() {
    let range = Uniform::from(0..20);
    let a: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();
    let b: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();

    let mut c = vec![0 as u64; 100];

    // Addition method 1
    let start = Instant::now();
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
    println!("Time taken {}", start.elapsed().as_secs_f32());

    // Addition method 2
    let mut d = vec![0 as u64; 100];
    unsafe {

        let am = ManuallyDrop::new(a);
        let bm = ManuallyDrop::new(b);
        let am_len = am.len();

        let a_ptr = am.as_ptr();
        let b_ptr = bm.as_ptr();

        let start = Instant::now();
        for i in 0..am_len {
            d[i] = *a_ptr.add(i) + *b_ptr.add(i);
        }
        println!("Time taken {}", start.elapsed().as_secs_f32());
    }

    assert_eq!(d, c);
}