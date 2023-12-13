use std::collections::VecDeque;
use std::cmp::PartialOrd;
use std::clone::Clone;

fn _merge_sort<T: PartialOrd + Clone + Copy>(a: &Vec<T>) -> Vec<T> {
    /*
        MERGE-SORT(A, p, r)
        if p < r
        then q = floor((p + r) / 2)
            merge_sort(A, p, q)
            merge_sort(A, q + 1, r)
            merge(A, p, q, r)
     */
    if a.len() > 1 {
        let q = a.len() / 2;
        let left = a[..q].to_vec();
        let right = a[q..].to_vec();
        let mut sorted_left = VecDeque::from(_merge_sort(&left));
        let mut sorted_right = VecDeque::from(_merge_sort(&right));
        _merge(&mut sorted_left, &mut sorted_right)
    } else {
        a.clone()
    }
}

fn _merge<T: PartialOrd + Clone + Copy>(l: &mut VecDeque<T>, r: &mut VecDeque<T>) -> Vec<T> {
    /*
        MERGE(A, p, q, r)
        n1 = q - p + 1
        n2 = r - q
        let mut L = vec![0..(n1 + 1)]
        let mut R = vec![0..(n2 + 1)]
        for i in 0..n1
            L[i] = A[p + i - 1]
        for j in 0..n2
            R[i] = A[q + j]
        i = j = 0
        for k in p..r
            loop {
                if L[i] <= R[j] {
                    A[k] = L[i]
                    i++
                } else {
                    A[k] = R[j]
                    j++
                }
            }
     */
    let mut return_vec: Vec<T> = Vec::new();
    let mut next_left = l.pop_front();
    let mut next_right = r.pop_front();
    loop {
        match next_left {
            Some(left) => match next_right {
                Some(right) => {
                    if left <= right {
                        return_vec.push(left);
                        next_left = l.pop_front();
                    } else {
                        return_vec.push(right);
                        next_right = r.pop_front();
                    }
                },
                None => {
                    return_vec.push(left);
                    next_left = l.pop_front();
                },
            },
            None => match next_right {
                Some(right) => {
                    return_vec.push(right);
                    next_right = r.pop_front();
                },
                None => break,
            }
        }
    }

    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_empty_array() {
        let a: Vec<u64> = vec![];
        let sorted = _merge_sort(&a);
        assert_eq!(sorted, []);
    }

    #[test]
    fn test_merge_sort_works_with_signed_values() {
        let a: Vec<i32> = vec![3, -5, 7, -2, 3, -54, 8, -9, 1, 0, 2];
        let sorted = _merge_sort(&a);
        assert_eq!(sorted, vec![-54, -9, -5, -2, 0, 1, 2, 3, 3, 7, 8]);
    }

    #[test]
    fn test_merge_sort_even_sized_array() {
        let a: Vec<u64> = vec![54, 6, 3, 6, 62, 5, 252, 62, 42, 54];
        let sorted = _merge_sort(&a);
        assert_eq!(sorted, vec![3, 5, 6, 6, 42, 54, 54, 62, 62, 252]);
    }

    #[test]
    fn test_merge_sort_odd_sized_array() {
        let a: Vec<u64> = vec![54, 6, 3, 6, 62, 5, 252, 62, 42];
        let sorted = _merge_sort(&a);
        assert_eq!(sorted, vec![3, 5, 6, 6, 42, 54, 62, 62, 252]);
    }
}
