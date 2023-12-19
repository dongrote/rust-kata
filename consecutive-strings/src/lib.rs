use std::collections::VecDeque;

pub fn longest_consec(strarr: &[&str], k: usize) -> Option<String> {
  let n = strarr.len();
  if k == 0 || n == 0 || k > n {
    return Option::None;
  }

  let mut longest = String::new();
  let mut candidate_strings: VecDeque<&str> = VecDeque::new();
  for i in 0..k-1 {
    candidate_strings.push_back(strarr[i]);
  }

  for i in k-1..n {
    candidate_strings.push_back(strarr[i]);

    {
      let mut consecutive = String::new();
      for s in &candidate_strings {
        consecutive.push_str(s);
      }
  
      if consecutive.len() > longest.len() {
        longest = consecutive;
      }  
    }

    let _ = candidate_strings.pop_front().unwrap_or_default();
  }

  Some(longest)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_longest_consec() {
    let strarr = ["abc", "defghi", "jklm"];
    assert_eq!(longest_consec(&strarr, 1).unwrap(), "defghi");
    assert_eq!(longest_consec(&strarr, 2).unwrap(), "defghijklm");
    assert_eq!(longest_consec(&strarr, 3).unwrap(), "abcdefghijklm");
  }

  #[test]
  fn test_longest_consec_none() {
    // strarr len 0
    assert_eq!(longest_consec(&[], 2), Option::None);
    // k <= 0
    assert_eq!(longest_consec(&["a", "b", "c"], 0), Option::None);
    // k > strarr len
    assert_eq!(longest_consec(&["a"], 2), Option::None);
  }
}
