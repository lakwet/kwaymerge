#[inline]
pub fn copy_nonoverlapping<T>(
    source: &mut [T],
    destination: &mut [T],
    length: usize,
) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            source.as_ptr(),
            destination.get_unchecked_mut(0),
            length,
        );
    }
}

fn is_sorted_and_unique(arr: &Vec<usize>) -> bool {
  for i in 0..arr.len() - 1 {
    if arr[i] >= arr[i + 1] {
      return false;
    }
  }

  true
}

pub fn check_separators(separators: &Vec<usize>, arr_len: usize) {
  // Check length is at least 2.
  // Check it is sorted and each element is unique.
  // Check first element is 0.
  // Check last element is the array size.
  if separators.len() < 2 {
    panic!("Separators length must be at least 2.");
  } else if !is_sorted_and_unique(separators) {
    panic!("Separators must be sorted and each element must be unique.");
  } else if separators[0] != 0 {
    panic!("First element in separators must be 0.");
  } else if separators[separators.len() - 1] != arr_len {
    panic!("Last element in separators must be the array size.");
  }
}
