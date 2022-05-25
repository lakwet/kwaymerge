use std::fmt;

use super::utils::{check_separators};

#[derive(Debug)]
pub struct TreeNode<'a, T: Copy + PartialOrd + std::fmt::Debug> {
  value: Option<T>,
  left: usize,
  right: usize,
  is_leaf: bool,
  data: &'a mut [T],
  data_index: usize,
}

impl<'a, T: Copy + PartialOrd + std::fmt::Debug> TreeNode<'a, T> {
  fn new_leaf(data: &'a mut [T]) -> TreeNode<T> {
    TreeNode {
      value: Some(data[0]),
      left: 0,
      right: 0,
      is_leaf: true,
      data,
      data_index: 1,
    }
  }

  fn new(
    value: Option<T>,
    left: usize,
    right: usize,
  ) -> TreeNode<'a, T> {
    TreeNode {
      value,
      left,
      right,
      is_leaf: false,
      data: &mut [],
      data_index: 0,
    }
  }
}

impl<'a, T: Copy + PartialOrd + std::fmt::Debug> fmt::Display for TreeNode<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_leaf {
      write!(f, "LEAF -- Value: {:?}; Data: {:?}; Index: {}\n", self.value, self.data, self.data_index)
    } else {
      write!(f, "NODE -- Value: {:?}, Left: {}, Right: {}\n", self.value, self.left, self.right)
    }
  }
}

#[derive(PartialEq, Debug)]
enum Candidate {
  IsLeft,
  IsRight,
  IsNone,
}

#[derive(Debug)]
pub struct TournamentTree<'a, T: Copy + PartialOrd + std::fmt::Debug> {
  pub tree: Vec<TreeNode<'a, T>>,
  pub head: usize,
}

impl<'a, T: Copy + PartialOrd + std::fmt::Debug> TournamentTree<'a, T> {
  pub fn new(slices: Vec<&'a mut [T]>) -> TournamentTree<T> {
    let mut tree = Vec::<TreeNode<T>>::new();

    for slice in slices.into_iter() {
      let node = TreeNode::new_leaf(slice);
      tree.push(node);
    }

    TournamentTree {
      tree,
      head: 0,
    }
  }

  pub fn init(&mut self) {
    let mut start = 0;
    let mut next_start = self.tree.len();
    let mut current = next_start;
    let mut nb_elements = current - start;

    while nb_elements > 1 {
      let is_odd = nb_elements % 2 == 1;

      for i in 0..nb_elements / 2 {
        let left = start + 2 * i;
        let right = start + 2 * i + 1;
        let (min, candidate) = match (self.tree[left].value, self.tree[right].value) {
          (None, None) => (None, Candidate::IsNone),
          (None, Some(x)) => (Some(x), Candidate::IsRight),
          (Some(x), None) => (Some(x), Candidate::IsLeft),
          (Some(x), Some(y)) => if x <= y {
            (Some(x), Candidate::IsLeft)
          } else {
            (Some(y), Candidate::IsRight)
          },
        };
        let node = TreeNode::new(min, left, right);
        self.tree.push(node);
        current += 1;

        match candidate {
          Candidate::IsNone => {},
          Candidate::IsLeft => {
            self.update(left);
          },
          Candidate::IsRight => {
            self.update(right);
          },
        }
      }

      if is_odd {
        let target = next_start - 1;
        let min = self.tree[target].value;
        let node = TreeNode::new(min, target, target);

        self.tree.push(node);
        current += 1;

        self.update(target);
      }

      start = next_start;
      next_start = current;
      nb_elements = next_start - start;
    }

    self.head = current - 1;
  }

  fn cmp_and_update(&mut self, index: usize) {
    let left_value = self.tree[self.tree[index].left].value;
    let right_value = self.tree[self.tree[index].right].value;

    let (min, candidate) = match (left_value, right_value) {
      (None, None) => (None, Candidate::IsNone),
      (None, Some(x)) => (Some(x), Candidate::IsRight),
      (Some(x), None) => (Some(x), Candidate::IsLeft),
      (Some(x), Some(y)) => if x <= y {
        (Some(x), Candidate::IsLeft)
      } else {
        (Some(y), Candidate::IsRight)
      },
    };

    self.tree[index].value = min;

    match candidate {
      Candidate::IsNone => {},
      Candidate::IsLeft => {
        self.update(self.tree[index].left);
      },
      Candidate::IsRight => {
        self.update(self.tree[index].right);
      },
    }
  }

  fn update(&mut self, index: usize) {
    if self.tree[index].is_leaf {
      let is_empty = self.tree[index].data_index == self.tree[index].data.len();

      if is_empty {
        self.tree[index].value = None;
      } else {
        let value = self.tree[index].data[self.tree[index].data_index];
        self.tree[index].value = Some(value);
        self.tree[index].data_index += 1;
      }
    } else {
      self.cmp_and_update(index);
    }
  }

  pub fn get_next(&mut self) -> Option<T> {
    let result = self.tree[self.head].value;

    if result.is_none() {
      // Short circuit, no need to update the tree, since it is empty.
      return result;
    }

    self.update(self.head);

    result
  }
}

impl<'a, T: Copy + PartialOrd + std::fmt::Debug> fmt::Display for TournamentTree<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, node) in self.tree.iter().enumerate() {
      print!("ID {} -- {}", i, node);
    }
    write!(f, "")
  }
}

fn merge_tree<T: Copy + PartialOrd + std::fmt::Debug>(arr: &mut [T], slices: Vec<&mut [T]>) {
  let mut tree = TournamentTree::new(slices);
  tree.init();

  let mut i = 0;
  while let Some(next) = tree.get_next() {
    arr[i] = next;
    i += 1;
  }
}

pub fn tournament_tree<T: Copy + PartialOrd + std::fmt::Debug>(
  arr: &mut [T],
  separators: &Vec<usize>,
) {
  check_separators(separators, arr.len());

  if separators.len() == 2 {
    return;
  }

  let mut slices = Vec::<&mut [T]>::with_capacity(separators.len() - 1);
  let mut copy = arr.to_vec();
  let mut rest = copy.as_mut_slice();

  for i in 1..separators.len() {
    let (fst, snd) = rest.split_at_mut(separators[i] - separators[i - 1]);
    slices.push(fst);
    rest = snd;
  }

  merge_tree(arr, slices);
}
