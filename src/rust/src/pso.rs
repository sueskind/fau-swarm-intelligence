use crate::math::{addv, randv, randvv};

pub fn pso(func: fn(Vec<f64>) -> f64, dim: usize, n: usize, s: f64) {
    let mut x: Vec<Vec<f64>> = randvv(n, dim, -s, s);
    let mut v: Vec<Vec<f64>> = (0..n).map(|_| vec![0.; dim]).collect();
    let mut p: Vec<Vec<f64>> = x.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
