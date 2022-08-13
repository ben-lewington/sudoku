#[cfg(not(target_family = "wasm"))]
use rand::prelude::*;
use wasm_bindgen::prelude::*;

fn sequential_range<const N: usize>() -> [usize; N * N]
where
    [(); N * N]: Sized,
{
    let mut a = [0; N * N];
    let _ = a
        .iter_mut()
        .enumerate()
        .map(|(i, v)| *v = i + 1)
        .collect::<()>();
    a
}

#[cfg(not(target_family = "wasm"))]
pub(crate) fn shuffle_range<const N: usize>() -> [usize; N * N]
where
    [(); N * N]: Sized,
{
    let mut r = thread_rng();
    let mut a = sequential_range::<N>();
    a.shuffle(&mut r);
    a
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[cfg(target_family = "wasm")]
fn rand_between(min: usize, max: usize) -> usize {
    let r = unsafe { random() };
    (r * ((max - min) as f64)).floor() as usize + min
}

#[cfg(not(target_family = "wasm"))]
fn rand_between(min: usize, max: usize) -> usize {
    let mut r = thread_rng();
    (r.gen::<f64>() * (max as f64 - min as f64)).floor() as usize + min
}

#[cfg(target_family = "wasm")]
pub(crate) fn shuffle_range<const N: usize>() -> [usize; N * N]
where
    [(); N * N]: Sized,
{
    let m = N * N;
    let mut shuffled = sequential_range::<N>();
    (0..=m - 1).map(|i| {
        let j = rand_between(i + 1, m - 1);
        shuffled.swap(i, j);
    });
    shuffled
}
