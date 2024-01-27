mod utils;
mod complex;

use complex::Complex;

use js_sys::Float64Array;
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::{
    sync::Mutex, 
    f64::consts::PI,
};


// Assumes array of complex numbers
fn fft(arr: &mut [Complex], start: usize, stop: usize, step: usize) {
    let n = ((stop - start) / step);
    let l = 31 ^ n.leading_zeros() as usize;

    lazy_static! {
        static ref R: Mutex<Vec<Complex>> = Mutex::new(vec![1.0, 1.0].into_iter().map(Complex::from).collect());
        static ref rt: Mutex<Vec<Complex>> = Mutex::new(vec![1.0, 1.0].into_iter().map(Complex::from).collect());
    }

    let mut k = 2;

    while k < n {
        R.lock().unwrap().resize(n, Complex::from(0.0));
        rt.lock().unwrap().resize(n, Complex::from(0.0));

        let x = Complex::from_polar(1.0, PI / (k as f64));

        for i in k..2*k {
            R.lock().unwrap()[i] = if (i & 1) == 1 {
                R.lock().unwrap()[i >> 1] * x
            } else {
                R.lock().unwrap()[i >> 1]
            };

            rt.lock().unwrap()[i] = R.lock().unwrap()[i];
        }

        k *= 2;
    }

    let mut rev: Vec<usize> = vec![0; n];

    for i in 0..n { rev[i] = (rev[i >> 1] | ((i & 1) << l) ) / 2 };
    for i in 0..n {
        if i < rev[i] {
            // Bruhest swap 
            let tmp = arr[start + rev[i] * step];
            arr[start + rev[i] * step] = arr[start + i * step];
            arr[start + i * step] = tmp;
        }
    }

    k = 1;

    while k < n {
        for i in (0..n).step_by(2*k) {
            for j in 0..k {
                // This might be sped up by "hand rolling"
                let i1 = start + (i + j + k) * step;
                let i2 = start + (i + j) * step;
                let z = rt.lock().unwrap()[j + k] * arr[i1];

                arr[i1] = arr[i2] - z;
                arr[i2] += z;
            }
        }
        k *= 2;
    }
}

#[wasm_bindgen]
// Arr should be in row major order
pub fn fft2(rows: usize, cols: usize, arr: &mut [f64]) -> Float64Array {
    let mut complex_mat: &mut [Complex] = unsafe {
        std::slice::from_raw_parts_mut(arr.as_mut_ptr() as *mut Complex, arr.len() / 2)
    };


    for row in 0..rows {
        fft(&mut complex_mat, row * cols, row * cols + cols, 1);
    }

    for col in 0..cols {
        fft(&mut complex_mat, col, rows * cols, cols);
    }

    let f64_slice: &[f64] = unsafe {
        std::slice::from_raw_parts(complex_mat.as_ptr() as *const f64, complex_mat.len() * 2)
    };

    Float64Array::from(&f64_slice[..])
}
