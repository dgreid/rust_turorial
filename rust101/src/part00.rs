use std::ops::Add;

fn read_vec() -> Vec<i32> {
    vec![3, 4, 6, -19]
}

fn vec_min<T: PartialOrd>(v: &Vec<T>) -> Option<&T> {
    if v.is_empty() { return None; }
    let mut min = Some(&v[0]);
    for e in v {
        min = min.map(|val| {
            if e < val { e } else { val }
        });
    }
    min
}

fn vec_sum<T: Copy + Add<T, Output=T>>(v: &Vec<T>) -> Option<T> {
    let mut sum = None;
    for e in v.iter() {
        match sum {
            None => {
                sum = Some(*e);
            },
            Some(val) => {
                let newsum = *e + val;
                sum = Some(newsum);
            },
        }
    }
    sum
}

pub fn main() {
    let vec1 = read_vec();
    let min = vec_min(&vec1);
    min.map(|val| println!("Vec min {}", val));
    let sum = vec_sum(&vec1);
    sum.map(|val| println!("Vec sum {}", val));
    let vecf = vec![0.3, 0.5, 4.3, -0.3, -0.5];
    let minf = vec_min(&vecf);
    minf.map(|val| println!("Vec min {}", val));
}
