use rand::{thread_rng, Rng};
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub fn random_array_u64(size: usize) -> Vec<u64> {
  (0..size)
    .into_par_iter()
    // .map(|_| thread_rng().gen::<u64>())
    .map(|_| thread_rng().gen_range(0, 1000))
    .collect::<Vec<u64>>()
}

pub fn random_separators(range: usize, separators_count: usize) -> Vec<usize> {
  let mut rng = thread_rng();
  (0..separators_count)
    .into_iter()
    .map(|_| rng.gen_range(1, range))
    .collect::<Vec<usize>>()
}
