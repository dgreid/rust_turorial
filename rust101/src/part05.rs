use std::{cmp, ops};

#[derive(Debug)]
#[derive(Clone)]
struct BigInt {
    pub data: Vec<u64>,
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }
}

impl BigInt {
    pub fn from_vec(v: &Vec<u64>) -> Self {
        let mut newbi = BigInt::new(0);
        for e in v {
            newbi.data.push(*e);
        }
        while let Some(val) = newbi.data.pop() {
            if val != 0 {
                newbi.data.push(val);
                break;
            }
        }
        newbi
    }
}

impl BigInt {
    pub fn num_digits(&self) -> usize {
        self.data.len()
    }
}

impl BigInt {
    pub fn num_non_zero_digits(&self) -> usize {
        let mut ret = 0;
        for val in self.data.iter() {
            if *val != 0 {
                ret = ret + 1;
            }
        }
        ret
    }
}

fn overflowing_add(a: u64, b: u64, carry: bool) -> (u64, bool) {
    let mut sum = a.wrapping_add(b);
    let mut carry_out = false;
    if sum <= a && b > 0 && a > 0 {
        carry_out = true;
    }

    if carry {
        sum = sum.wrapping_add(1);
        if sum == 0 {
            carry_out = true;
        }
    }
    (sum, carry_out)
}

trait Minimum {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl Minimum for BigInt {
    fn min<'a>(&'a self, b: &'a Self) -> &Self {
        if self.num_digits() < b.num_digits() {
            self
        } else if b.num_digits() < self.num_digits() {
            b
        } else {
            for (self_elem, b_elem) in self.data.iter().rev().zip(b.data.iter().rev()) {
                if b_elem < self_elem {
                    return b;
                } else if self_elem < b_elem {
                    return self;
                }
            }
            self
        }

    }
}

fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
    if v.is_empty() { return None; }
    let mut min = Some(&v[0]);
    for bi in v {
        min = min.map(|min_bi| min_bi.min(bi));
    }
    min
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a, 'b> ops::Add<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn add(self, other: &'a BigInt) -> Self::Output {
        let mut result_bi = BigInt::new(0);
        let max_len = cmp::max(self.data.len(), other.data.len());
        let mut carry = false;

        for i in 0..max_len {
            let addend1 = if i < self.data.len() { self.data[i] } else { 0 };
            let addend2 = if i < other.data.len() { other.data[i] } else { 0 };
            let (sum, new_carry) = overflowing_add(addend1, addend2, carry);
            result_bi.data.push(sum);
            carry = new_carry;
        }

        if carry {
            result_bi.data.push(1);
        }

        result_bi
    }
}

impl ops::Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, other: BigInt) -> Self::Output {
        (&self + &other)
    }
}

// Iter

pub struct Iter<'a> {
    nums: &'a Vec<u64>,
    idx: usize,
}

impl BigInt {
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter { nums: &self.data, idx: self.data.len() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 0 {
            None
        } else {
            self.idx = self.idx - 1;
            Some(&self.nums[self.idx])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigInt;
    use super::Minimum;
    use super::overflowing_add;
    use super::vec_min;

    #[test]
    fn test_iter() {
        let vec: Vec<u64> = vec![8,4,0,5];
        let v2 = BigInt::from_vec(&vec![8,4,0,5,0,0,0]);
        for (b, v) in v2.iter().zip(vec.iter().rev()) {
            assert_eq!(b, v);
        }
    }

    #[test]
    fn iter() {
        let v2 = BigInt::from_vec(&vec![8,4,0,5]);
        let mut iter = v2.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&8));
    }

    #[test]
    fn test_counts() {
        let v2 = BigInt::from_vec(&vec![8,4,0,5,0,0,0]);
        let v3 = v2.clone();
        assert_eq!(4, v2.num_digits());
        assert_eq!(4, v3.num_digits());
        assert_eq!(3, v2.num_non_zero_digits());
        assert_eq!(3, v3.num_non_zero_digits());
    }

    #[test]
    fn test_min() {
        let v = BigInt::new(5);
        let v2 = BigInt::from_vec(&vec![8,4,0,5,0,0,0]);
        let a = BigInt::from_vec(&vec![1,3,4]);
        let b = BigInt::from_vec(&vec![2,3,4]);

        assert_eq!(&a, a.min(&b));
        assert_eq!(&a, b.min(&a));

        let bvec = vec![v2, a, b, v];
        assert_eq!(Some(&bvec[3]), vec_min(&bvec));
    }

    #[test]
    fn test_overflowing_add() {
        assert_eq!(overflowing_add(10, 100, false), (110, false));
        assert_eq!(overflowing_add(10, 100, true), (111, false));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, false), (0, true));
        assert_eq!(overflowing_add(1 << 63, 1 << 63, true), (1, true));
        assert_eq!(overflowing_add(u64::max_value(), 1, false), (0, true));
        assert_eq!(overflowing_add(u64::max_value() - 1, 1, true), (0, true));
    }

    #[test]
    fn test_bigint_add() {
        let b1 = BigInt::new(500);
        let b2 = BigInt::from_vec(&vec![0, 1]);

        assert_eq!(&b1 + &b2, BigInt::from_vec(&vec![500, 1]));
        assert_eq!(b1 + b2, BigInt::from_vec(&vec![500, 1]));
    }
}
