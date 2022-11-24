use std::{rc::Rc, time::Instant};

use rayon::prelude::{IntoParallelRefMutIterator, IndexedParallelIterator, ParallelIterator, IntoParallelRefIterator, IntoParallelIterator};

const N: usize = 500_000;
const ITERS: usize = 60_000;

fn main() {
    println!("Hello");
    // let a = [1.; N];
    let a = vec![1.; N];
    let b = vec![2.; N];
    // let a = Rc::new(vec![1.; N]);
    // let b = Rc::new(vec![2.; N]);

    // let mut c = vec![0.; N];
 

    let now = Instant::now();

    // for () in z

    // let c: Vec<f64> = (0..N).into_par_iter().map(|i| { a[i] + b[i] }).collect();
    let c: Vec<f64> = (0..N).into_iter().map(|i| { a[i] + b[i] }).collect();

    // c.par_iter_mut().enumerate().for_each(|(i, p)| *p = a[i] + b[i]);
    // c.iter_mut().enumerate().for_each(|(i, p)| *p = a[i] + b[i]);

    // for i in 0..N {
    //     c[i] = a[i] + b[i];
    // }

    let _ = Rc::new(c);

    println!(
        "{} secs, {} elems cloned {} times",
        now.elapsed().as_secs_f64(),
        N,
        ITERS
    );
}
