// simd_add.rs
#![feature(test)]
#![feature(portable_simd)]

use rand::{
    distributions::Uniform,
    thread_rng, Rng,
};
use std::{simd::f32x4, time::Instant};

macro_rules! assert_equal_len {
    ($a:ident, $b: ident) => {
        assert!(
            $a.len() == $b.len(),
            "add_assign: dimension mismatch: {:?} += {:?}",
            ($a.len(),),
            ($b.len(),)
        );
    };
}

// element-wise addition
fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x += *y;
    }
}

// simd accelerated addition
fn simd_add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    let size = xs.len() as isize;
    let chunks = size / 4;

    // pointer to the start of the vector data
    let p_x: *mut f32 = xs.as_mut_ptr();
    let p_y: *const f32 = ys.as_ptr();

    // sum excess elements that don't fit in the simd vector
    for i in (4 * chunks)..size {
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            // offset by i elements
            *p_x.offset(i) += *p_y.offset(i);
        }
    }

    // treat f32 vector as an simd f32x4 vector
    let simd_p_x = p_x as *mut f32x4;
    let simd_p_y = p_y as *const f32x4;

    // sum "simd vector"
    for i in 0..chunks {
        unsafe {
            *simd_p_x.offset(i) += *simd_p_y.offset(i);
        }
    }
}

fn main() {
    let mut a: Vec<f32> = Vec::with_capacity(10_000);
    let mut b: Vec<f32> = Vec::with_capacity(10_000);

    for _ in 0..10_000 {
        a.push(thread_rng().sample(Uniform::from(0.0..1.0)));
        b.push(thread_rng().sample(Uniform::from(0.0..1.0)));
    }

    let start = Instant::now();
    for _ in 0..100 {
        add_assign(&mut a, &mut b);
    }
    println!("Time taken vanilla {:?}", start.elapsed().as_secs_f32());

    let start = Instant::now();
    for _ in 0..100 {
        simd_add_assign(&mut a, &mut b);
    }
    println!("Time taken simd {:?}", start.elapsed().as_secs_f32());
}
