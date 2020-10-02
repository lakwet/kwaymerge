use super::super::algorithms::k_way_merge::k_way_merge;
use super::super::algorithms::k_way_ping_pong_merge::k_way_ping_pong_merge;
use super::helpers::speed_test_aux;

#[test]
fn speed_test() {
  let array_size = [
    5_000_000,
  ];

  let runs = 5;
  let with_check = true;
  let names = vec!["Naive", "Ping pong"];

  println!("Runs: {}", runs);

  for name in names.iter() {
    print!("\t\t\t{}", name);
  }
  println!();

  for size in array_size.iter() {
    print!("Array size: {}", *size);

    speed_test_aux(&|arr, sep| k_way_merge(arr, sep), runs, *size, with_check);
    speed_test_aux(&|arr, sep| k_way_ping_pong_merge(arr, sep), runs, *size, with_check);

    println!();
  }
}
