/// Return the p-norm between two vectors.
/// The special case of p=2 is euclidean distance.
pub fn pnorm(p: f32, u: &[f32], v: &[f32]) -> f32 {
    if u.len() != v.len() {
        panic!("unequal vector lengths {} != {}", u.len(), v.len());
    }
    let mut dot = 0.0;
    for (x, y) in u.iter().zip(v) {
        dot += (x - y).powf(p)
    }
    dot.powf(1.0 / p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let u = vec![];
        let v = vec![];
        let d = pnorm(2.0, &u, &v);
        let expect = 0.0;
        assert!((d - expect).abs() < 1e-6);
    }

    #[test]
    fn simple() {
        let u = vec![0.0, 0.0, 0.0, 0.0];
        let v = vec![1.0, 1.0, 1.0, 1.0];
        let d = pnorm(2.0, &u, &v);
        let expect = 2.0;
        assert!((d - expect).abs() < 1e-6);
    }
}
