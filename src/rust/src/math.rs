use rand::{distributions::Uniform, Rng};

pub fn randv(n: usize, low: f64, high: f64) -> Vec<f64> {
    let range = Uniform::from(low..high);
    rand::thread_rng().sample_iter(&range).take(n).collect()
}

pub fn randvv(n: usize, m: usize, low: f64, high: f64) -> Vec<Vec<f64>> {
    let range = Uniform::from(low..high);
    (0..n).map(|_| rand::thread_rng().sample_iter(&range).take(m).collect()).collect()
}

pub fn fargmin(func: fn(Vec<f64>) -> f64, x: Vec<Vec<f64>>) -> Vec<f64> {
    let min_xi = 0.;
    let min_yi = f64::INFINITY;
    for xi in x.iter() {}
}

pub fn addv(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addv() {
        let a: Vec<f64> = vec![0., 1., 2., 3., 4.];
        let b: Vec<f64> = vec![1., 2., 3., 4., 5.];
        assert_eq!(vec![1., 3., 5., 7., 9.], addv(a, b));
    }
}
