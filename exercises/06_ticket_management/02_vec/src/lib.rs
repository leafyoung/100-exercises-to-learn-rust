// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // index in usize: `n as usize + 1` can't overflow usize on >=32-bit targets,
    // avoiding the u32 overflow that `(n + 1) as usize` hits at n == u32::MAX.
    let mut seq: Vec<u32> = Vec::with_capacity(n as usize + 1);
    seq.push(0);
    if n == 0 {
        return 0;
    }
    seq.push(1);

    for i in 2..=n {
        // saturating_add: fib overflows u32 at n=48; saturate instead of
        // panicking (debug) or silently wrapping (release).
        let next = seq[(i - 1) as usize].saturating_add(seq[(i - 2) as usize]);
        seq.push(next);
    }
    seq[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
