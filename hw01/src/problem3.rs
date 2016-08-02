/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let vsize = n as usize;
    let mut not_prime = vec![false; vsize];
    let mut result = Vec::new();

    if vsize < 2 {
        return result;
    }

    for i in 2..n {
        if not_prime[i as usize] {
            continue;
        }

        result.push(i);

        let mut j = i * i;
        while j < n {
            not_prime[j as usize] = true;
            j = j + i;
        }
    }

    result
}
