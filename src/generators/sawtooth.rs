use rayon::slice::ParallelSliceMut;

use super::array::random_array_u64;
use super::separators::random_separators;

fn sort_slices(arr: &mut [u64], separators: &[usize]) {
  let mut buf = arr;
  for r in 0..separators.len() - 1 {
    let (fst, snd) = buf.split_at_mut(separators[r + 1] - separators[r]);
    buf = snd;
    fst.par_sort_unstable();
  }
  buf.par_sort_unstable();
}

pub fn random_sawtooth_array(size: usize) -> (Vec<u64>, Vec<usize>) {
  let mut arr = random_array_u64(size);
  let mut separators_count = (size as f64).log2() as usize;
  if separators_count > 2 {
    separators_count -= 2;
  }
  let mut separators = random_separators(size, separators_count);

  separators.sort_unstable();
  separators.insert(0, 0);
  separators.push(size);
  separators.dedup();

  sort_slices(arr.as_mut_slice(), separators.as_slice());

  (arr, separators)
}

pub fn random_bounds_sawtooth_array(size: usize) -> (Vec<u64>, Vec<usize>) {
  let mut separators = Vec::new();
  let mut arr = Vec::with_capacity(size);
  let mut value = 1;
  let threshold = (size as f64 / (size as f64).log2()) as usize;

  for i in 0..size {
    if value > threshold {
      value = 1;
      separators.push(i);
    }
    arr.push(value as u64);
    value += 1;
  }

  separators.insert(0, 0);
  separators.push(size);

  (arr, separators)
}
