use super::super::algorithms::k_way_merge::k_way_merge;
use super::super::algorithms::k_way_ping_pong_merge::k_way_ping_pong_merge;
use super::super::algorithms::tournament_tree::tournament_tree;
use super::super::generators::sawtooth::{
  random_bounds_sawtooth_array, random_sawtooth_array,
};
use super::helpers::speed_test_aux;

fn get_generators(
) -> Vec<(&'static str, &'static dyn Fn(usize) -> (Vec<u64>, Vec<usize>))> {
  vec![
    ("-- random separators:  ", &random_sawtooth_array),
    ("-- uniform generators: ", &random_bounds_sawtooth_array),
  ]
}

#[test]
fn speed_test() {
  let generators = get_generators();
  let names = vec!["Naive", "Ping pong", "Tournament tree"];

  let array_size = [
    1_000,
    10_000,
    50_000,
    1_000_000,
  ];
  let with_check = true;
  let runs = 10;

  println!("Runs: {}", runs);
  println!("With check: {}", with_check);
  for name in names.iter() {
    print!("\t\t\t{}", name);
  }
  println!();
  for size in array_size.iter() {
    println!("Array size: {}", *size);

    for (gen_name, gen) in generators.iter() {
      print!("{}", gen_name);
      speed_test_aux(&|arr, sep| k_way_merge(arr, sep), gen, runs, *size, with_check);
      speed_test_aux(&|arr, sep| k_way_ping_pong_merge(arr, sep), gen, runs, *size, with_check);
      speed_test_aux(&|arr, sep| tournament_tree(arr, sep), gen, runs, *size, with_check);
      println!();
    }

  }
}
