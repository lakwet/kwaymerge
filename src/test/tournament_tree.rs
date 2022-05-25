use super::super::generators::array::random_array_u64;
use super::super::algorithms::tournament_tree::{
  tournament_tree, TournamentTree,
};

#[test]
fn test_tournament_tree_build_simple_tree() {
  let mut data = vec![1,2,3,4,3,4,5,6,3,7,8,2,3,4,5,6,7];
  let mut rest = data.as_mut_slice();
  let mut slices = Vec::new();
  let bounds = vec![0, 4, 8, 11, 17];

  for i in 1..bounds.len() {
    let (fst, snd) = rest.split_at_mut(bounds[i] - bounds[i - 1]);
    slices.push(fst);
    rest = snd;
  }

  assert_eq!(slices[0], &[1,2,3,4]);
  assert_eq!(slices[1], &[3,4,5,6]);
  assert_eq!(slices[2], &[3,7,8]);
  assert_eq!(slices[3], &[2,3,4,5,6,7]);

  /*
                   1
          _________|_________
          |                 |
          2                 2
      ____|____         ____|____
      |        |        |        |
      3        3        3        3
      |        |        |        |
      4        4        7        4
      N        5        8        5
               6        N        6
               N                 7
                                 N
  */
  let mut tree = TournamentTree::new(slices);
  tree.init();
  assert_eq!(tree.head, 6);

  println!("{}", tree);

  let mut result = Vec::new();
  while let Some(next) = tree.get_next() {
    result.push(next);
  }

  assert_eq!(result, vec![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 7, 8]);
  println!("{:?}", result);
}
