pub fn insertion_sort(values: &Vec<i16>) -> Vec<i16> {
  let mut sorted: Vec<i16> = Vec::new();
  if values.len() < 2 {
    return Vec::clone(values);
  }

  for value in values {
    println!("value: {}", value);
    let mut i: usize = 0;
    while i < sorted.len() {
      if value < &sorted[i] {
        break;
      }
      i = i + 1;
    }

    sorted.insert(i, *value);
  }

  sorted
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insertion_sort_works_with_empty_vector() {
    assert_eq!(insertion_sort(&vec![]), vec![]);
  }

  #[test]
  fn insertion_sort_works_with_reversed_vector() {
    assert_eq!(insertion_sort(&vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
  }

  #[test]
  fn insertion_sort_works_with_identity_vector() {
    assert_eq!(insertion_sort(&vec![1,1,1,1,1,1]), vec![1,1,1,1,1,1]);
  }

  #[test]
  fn insertion_sort_works() {
    assert_eq!(insertion_sort(&vec![2, 1, 3, 0]), vec![0, 1, 2, 3]);
  }
}
