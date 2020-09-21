use std::time::Instant;

use super::super::generators::random_array_u64;
use super::super::algorithms::k_way_merge::k_way_merge;

#[allow(dead_code)]
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

#[test]
fn speed_test() {
  let array_size = [
    100_000_000,
  ];

  let runs = 10;
  let mut nanos: Vec<u64> = Vec::with_capacity(runs);

  for size in array_size.iter() {
    for _ in 0..runs {
      let mut arr = random_array_u64(*size);

      let start = Instant::now();
      k_way_merge(&mut arr);
      let ns: u64 = start.elapsed().as_nanos() as u64;
      nanos.push(ns);
    }

    let sum: u64 = nanos.iter().sum();
    let mean: u64 = sum / runs as u64;
    let std_dev: f32 = std_deviation(&nanos, mean, *size);
    let per_item: f32 = (mean as f64 / *size as f64) as f32;

    // \u{1b} => escape for terminal
    // 0 => no color
    // 0;30 => gray
    // 0;31 => red
    // 1;31 => red
    // 0;32 => green
    // 0;33 => brown
    // 0;34 => blue
    // 1;34 => light blue
    // 0;37 => light gray

    // print time, standard deviation and time per item
    println!("Array size: {}", *size);
    println!(
      "\u{1b}[0;32m{}us\u{1b}[0m\t\u{1b}[1;31m{:.0}ns\u{1b}[0m\t(\u{1b}[0;\
       33m{:.2}ns\u{1b}[0m)\t",
      mean / 1000,
      std_dev,
      per_item
    );
  }
}
