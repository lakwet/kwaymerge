use super::super::generators::array::random_array_u64;
use super::super::algorithms::k_way_ping_pong_merge::{
  merge_out_of_place_2, merge_slices,
};
use super::super::algorithms::utils::copy_nonoverlapping;

#[test]
fn test_ping_pong_merge_out_of_place_2() {
  let n = 20;
  let m = 30;
  let mut a = random_array_u64(n);
  let mut b = random_array_u64(m);
  let mut dest = vec![0; n + m];

  a.sort_unstable();
  b.sort_unstable();

  let mut check = vec![0; n + m];
  copy_nonoverlapping(a.as_mut_slice(), &mut check, n);
  copy_nonoverlapping(b.as_mut_slice(), &mut check[n..(n + m)], m);
  check.sort_unstable();

  merge_out_of_place_2(a.as_mut_slice(), b.as_mut_slice(), dest.as_mut_slice());

  assert_eq!(dest, check);
}

#[test]
fn test_ping_pong_merge_slices_even() {
  let n = 10;
  let mut a = random_array_u64(n); a.sort_unstable();
  let mut b = random_array_u64(n); b.sort_unstable();
  let mut c = random_array_u64(n); c.sort_unstable();
  let mut d = random_array_u64(n); d.sort_unstable();
  let mut source = vec![0; 4 * n];
  copy_nonoverlapping(a.as_mut_slice(), &mut source[0..n], n);
  copy_nonoverlapping(b.as_mut_slice(), &mut source[n..n * 2], n);
  copy_nonoverlapping(c.as_mut_slice(), &mut source[n * 2..n * 3], n);
  copy_nonoverlapping(d.as_mut_slice(), &mut source[n * 3..n * 4], n);

  let mut dest = vec![0; 4 * n];
  let sep = vec![0, 10, 20, 30, 40];

  let new_sep = merge_slices(&mut source, &mut dest, sep);

  assert_eq!(new_sep, vec![0, 20, 40]);

  for i in 1..n * 2 {
    assert!(dest[i - 1] <= dest[i]);
  }
  for i in n * 2 + 1..n * 4 {
    assert!(dest[i - 1] <= dest[i]);
  }
}

#[test]
fn test_ping_pong_merge_slices_odd() {
  let n = 10;
  let mut a = random_array_u64(n); a.sort_unstable();
  let mut b = random_array_u64(n); b.sort_unstable();
  let mut c = random_array_u64(n); c.sort_unstable();
  let mut d = random_array_u64(n); d.sort_unstable();
  let mut e = random_array_u64(n); e.sort_unstable();
  let mut source = vec![0; 5 * n];
  copy_nonoverlapping(a.as_mut_slice(), &mut source[0..n], n);
  copy_nonoverlapping(b.as_mut_slice(), &mut source[n..n * 2], n);
  copy_nonoverlapping(c.as_mut_slice(), &mut source[n * 2..n * 3], n);
  copy_nonoverlapping(d.as_mut_slice(), &mut source[n * 3..n * 4], n);
  copy_nonoverlapping(e.as_mut_slice(), &mut source[n * 4..n * 5], n);

  let mut dest = vec![0; 5 * n];
  let sep = vec![0, 10, 20, 30, 40, 50];

  let new_sep = merge_slices(&mut source, &mut dest, sep);

  assert_eq!(new_sep, vec![0, 20, 40, 50]);

  for i in 1..n * 2 {
    assert!(dest[i - 1] <= dest[i]);
  }
  for i in n * 2 + 1..n * 4 {
    assert!(dest[i - 1] <= dest[i]);
  }
  for i in n * 4 + 1..n * 5 {
    assert!(dest[i - 1] <= dest[i]);
  }
  assert_eq!(e, &dest[n * 4..n * 5]);
}
