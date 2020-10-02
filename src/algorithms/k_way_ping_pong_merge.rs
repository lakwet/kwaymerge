use std::collections::VecDeque;

use super::utils::{check_separators, copy_nonoverlapping};

pub fn merge_out_of_place_2<T: Copy + PartialOrd + std::fmt::Debug>(
  a: &mut [T],
  b: &mut [T],
  dest: &mut [T],
) {
  let end_a = a.len();
  let end_b = b.len();
  let l = dest.len();
  assert_eq!(end_a + end_b, l);
  let mut i = 0;
  let mut j = 0;
  let mut k = 0;

  loop {
    if i == end_a {
      copy_nonoverlapping(&mut b[j..end_b], &mut dest[end_a + j..l], end_b - j + 1);
      break;
    } else if j == end_b {
      copy_nonoverlapping(&mut a[i..end_a], &mut dest[end_b + i..l], end_a - i + 1);
      break;
    }

    if a[i] < b[j] {
      dest[k] = a[i];
      i +=1;
    } else {
      dest[k] = b[j];
      j += 1;
    }

    k += 1;
  }
}

pub fn merge_slices<T: Copy + PartialOrd + std::fmt::Debug>(
  src: &mut [T],
  dest: &mut [T],
  separators: Vec<usize>,
) -> Vec<usize> {
  let slices_count = separators.len() - 1;
  let mut slices: Vec<&mut [T]> = Vec::with_capacity(slices_count);
  let mut buffer = src;

  for i in 1..slices_count + 1 {
    let (fst, snd) = buffer.split_at_mut(separators[i] - separators[i - 1]);
    slices.push(fst);
    buffer = snd;
  }

  slices.sort_unstable_by_key(|slice| slice.len());

  let new_sep_len = if slices.len() % 2 == 0 {
    slices.len() / 2 + 1
  } else {
    (slices.len() + 1) / 2 + 1
  };

  let mut cursor = 0;
  let mut new_separators: Vec<usize> = Vec::with_capacity(new_sep_len);
  new_separators.push(0);
  for (i, slice) in slices.iter().enumerate() {
    cursor += slice.len();
    if i % 2 == 1 {
      new_separators.push(cursor);
    }
  }
  if slices.len() % 2 != 0 {
    new_separators.push(cursor);
  }

  let mut dest_slices = VecDeque::with_capacity(new_sep_len - 1);
  let mut buffer = dest;
  for i in 1..new_separators.len() {
    let slice_size = new_separators[i] - new_separators[i - 1];
    let (fst, snd) = buffer.split_at_mut(slice_size);
    dest_slices.push_back(fst);
    buffer = snd;
  }
  if new_separators.len() % 2 == 0 {
    dest_slices.push_back(buffer);
  }

  let iteration = slices.len() / 2;
  for _ in 0..iteration {
    let a = slices.remove(0);
    let b = slices.remove(0);
    let dest = dest_slices.pop_front().unwrap();

    merge_out_of_place_2(a, b, dest);
  }

  if !slices.is_empty() {
    let a = slices.remove(0);
    let dest = dest_slices.pop_front().unwrap();
    copy_nonoverlapping(a, dest, a.len());
  }

  new_separators
}

pub fn k_way_ping_pong_merge<T: Copy + PartialOrd + std::fmt::Debug>(
  arr: &mut [T],
  separators: &Vec<usize>,
) {
  check_separators(separators, arr.len());
  let mut sep = separators.to_vec();

  let mut copy: Vec<T> = arr.to_vec();

  let mut ping = arr;
  let mut pong = copy.as_mut_slice();
  let mut odd = 0;

  while sep.len() > 2 {
    let (mut t1, mut t2) = if odd == 0 {
      (ping, pong)
    } else {
      (pong, ping)
    };

    sep = merge_slices(&mut t1, &mut t2, sep);

    odd = 1 - odd;

    if odd == 1 {
      ping = t1;
      pong = t2;
    } else {
      ping = t2;
      pong = t1;
    }
  }

  if  odd == 1 {
    copy_nonoverlapping(pong, ping, pong.len());
  }
}
