mod utils;

use js_sys::Float64Array;
use wasm_bindgen::prelude::*;
use ndarray::{arr3};


#[wasm_bindgen]
// Arr should be in row major order
pub fn fft2(rows: usize, cols: usize, arr: &[f64]) -> Float64Array {
    let res = arr.into_iter()
        .map(|x| x * x)
        .collect::<Vec<_>>();

    Float64Array::from(&res[..])
}
