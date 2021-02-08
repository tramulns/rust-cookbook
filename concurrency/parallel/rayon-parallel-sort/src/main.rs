/// Параллельная сортировка вектора
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn main() {
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5)
            .map(|_| rng.sample(&Alphanumeric))
            .map(char::from)
            .collect()
    });
    vec.par_sort_unstable();
}
