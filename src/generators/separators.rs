use rand::{thread_rng, Rng};

pub fn random_separators(range: usize, separators_count: usize) -> Vec<usize> {
  let mut rng = thread_rng();
  (0..separators_count)
    .into_iter()
    .map(|_| rng.gen_range(1, range))
    .collect::<Vec<usize>>()
}
