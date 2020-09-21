use std::time::Instant;

use rayon::slice::ParallelSliceMut;

use super::super::generators::{random_array_u64, random_separators};
use super::super::algorithms::k_way_merge::k_way_merge;

fn std_deviation(data: &Vec<u64>, mean: u64, size: usize) -> f32 {
  let variance = data
      .iter()
      .map(|value| {
          let diff =
              if mean >= *value { mean - *value } else { *value - mean };

          (diff * diff) as f64
      })
      .sum::<f64>()
      / size as f64;

  variance.sqrt() as f32
}

fn sort_slices(arr: &mut [u64], separators: &[usize]) {
  let mut buf = arr;
  for r in 0..separators.len() - 1 {
    let (fst, snd) = buf.split_at_mut(separators[r + 1] - separators[r]);
    buf = snd;
    fst.par_sort_unstable();
  }
  buf.par_sort_unstable();
}

fn get_array(size: usize) -> (Vec<u64>, Vec<usize>) {
  let mut arr = random_array_u64(size);
  let separators_count = (size as f64).log2() as usize;
  let mut separators = random_separators(size, separators_count);

  separators.sort_unstable();
  separators.insert(0, 0);
  separators.push(size);
  separators.dedup();

  sort_slices(arr.as_mut_slice(), separators.as_slice());

  (arr, separators)
}

#[test]
fn speed_test() {
  let array_size = [
    10_000_000,
  ];

  let runs = 10;
  let mut nanos: Vec<u64> = Vec::with_capacity(runs);

  println!("Runs: {}", runs);

  for size in array_size.iter() {

    for _ in 0..runs {
      let (mut arr, mut separators) = get_array(*size);

      let start = Instant::now();
      k_way_merge(&mut arr, &mut separators);
      let ns: u64 = start.elapsed().as_nanos() as u64;
      nanos.push(ns);

      for i in 0..*size - 1 {
        assert!(arr[i] <= arr[i + 1]);
      }
    }

    let sum: u64 = nanos.iter().sum();
    let mean: u64 = sum / runs as u64;
    let std_dev: f32 = std_deviation(&nanos, mean, *size);
    let per_item: f32 = (mean as f64 / *size as f64) as f32;

    // print time, standard deviation and time per item
    println!("Array size: {}", *size);
    println!(
      "\t\u{1b}[0;32m{}us\u{1b}[0m\t\u{1b}[1;31m{:.0}ns\u{1b}[0m\t(\u{1b}[0;\
       33m{:.2}ns\u{1b}[0m)\t",
      mean / 1000,
      std_dev,
      per_item
    );
  }
}
