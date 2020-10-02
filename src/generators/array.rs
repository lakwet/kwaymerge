use rand::{thread_rng, Rng};
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub fn random_array_u64(size: usize) -> Vec<u64> {
  (0..size)
    .into_par_iter()
    // .map(|_| thread_rng().gen::<u64>())
    .map(|_| thread_rng().gen_range(0, 1_000_000_000))
    .collect::<Vec<u64>>()
}
