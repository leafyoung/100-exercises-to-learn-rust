// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let slice: &'static mut [i32] = v.leak();

    let (slice_left, slice_right) = slice.split_at(slice.len() / 2);

    let handle_left = thread::spawn(move || {
        let mut res_left = 0;

        for v in slice_left.iter() {
            res_left += v;
        }
        res_left
    });

    let handle_right = thread::spawn(move || {
        let mut res_right = 0;
        for v in slice_right.iter() {
            res_right += v;
        }
        res_right
    });

    handle_left.join().unwrap() + handle_right.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
