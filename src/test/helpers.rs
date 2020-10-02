use std::time::Instant;

use super::super::generators::sawtooth::random_sawtooth_array;

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

pub fn speed_test_aux(
  merger: &dyn Fn(&mut [u64], &mut Vec<usize>) -> (),
  runs: usize,
  array_size: usize,
  with_check: bool,
) {
  let mut nanos: Vec<u64> = Vec::with_capacity(runs);

  for _ in 0..runs {
    let (mut arr, mut separators) = random_sawtooth_array(array_size);
    if with_check {
      let mut copy = arr.to_vec();

      let start = Instant::now();
      merger(&mut arr, &mut separators);
      let ns: u64 = start.elapsed().as_nanos() as u64;
      nanos.push(ns);

      copy.sort_unstable();
      assert_eq!(arr, copy);
      // for i in 0..array_size - 1 {
      //   assert!(arr[i] <= arr[i + 1]);
      // }
    } else {
      let start = Instant::now();
      merger(&mut arr, &mut separators);
      let ns: u64 = start.elapsed().as_nanos() as u64;
      nanos.push(ns);
    }
  }

  let sum: u64 = nanos.iter().sum();
  let mean: u64 = sum / runs as u64;
  let std_dev: f32 = std_deviation(&nanos, mean, array_size);
  let per_item: f32 = (mean as f64 / array_size as f64) as f32;

  // print time, standard deviation and time per item
  print!(
    "\t\u{1b}[0;32m{}us\u{1b}[0m\t\u{1b}[1;31m{:.0}ns\u{1b}[0m\t(\u{1b}[0;\
     33m{:.2}ns\u{1b}[0m)\t",
    mean / 1000,
    std_dev,
    per_item
  );
}
