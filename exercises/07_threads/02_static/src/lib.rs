// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!

use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    // Alternatively, we use manual
    // let mut slice_left = &slice[..(slice.len() / 2)];
    // let mut slice_right = &slice[(slice.len() / 2)..];
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
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
