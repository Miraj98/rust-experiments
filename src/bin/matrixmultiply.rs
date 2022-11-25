use std::time::Instant;

use matrixmultiply::sgemm;
use rand::{Rng, distributions::Uniform};

fn main() {
    let range = Uniform::from(0.0..20.0);
    let a: Vec<f32> = rand::thread_rng().sample_iter(&range).take(10000).collect();
    let b: Vec<f32> = rand::thread_rng().sample_iter(&range).take(10000).collect();
   let mut c = vec![0.; 10000];

   // Manual implemnentation
   let start1 = Instant::now();
   for rowa in 0..100 {
    for colb in 0..100 {
        for i in 0..100 {
            c[rowa * 100 + colb] += a[rowa * 100 + i] * b[i * 100 + colb];
        }
    }
   }
   println!("{} secs", start1.elapsed().as_secs_f32());

   let start2 = Instant::now();
   let mut d = vec![0.; 10000];
   unsafe {
        sgemm(100, 100, 100, 1., a.as_ptr(), 100, 1, b.as_ptr(), 100, 1, 0., d.as_mut_ptr(), 100, 1)
   }
   println!("{} secs", start2.elapsed().as_secs_f32());

   assert_eq!(c, d);
}