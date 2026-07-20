// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;

    thread::scope(|scope| {
        let h1 = scope.spawn(|| {
            let first = &v[..midpoint];
            let mut sum = 0;
            for x in first.iter() {
                sum += x;
            }
            sum
        });
        let h2 = scope.spawn(|| {
            let second = &v[midpoint..];
            let mut sum = 0;
            for x in second.iter() {
                sum += x;
            }
            sum
        });
        h1.join().unwrap() + h2.join().unwrap()
    })
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
