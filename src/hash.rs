use ndarray::{Array2, Ix, Ix2};
use ndarray_rand::{RandomExt, F32};
use rand::distributions::{Distribution, StandardNormal, Uniform};
use std::error::Error;

// TODO: What's the best way to share this?
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct RBP {
    vec_dim: Ix,
    hash_dim: Ix,
    kernel: Array2<f32>,
}

/// Current implementation uses [1]. However, [2] mentions that [3] not using
/// the offset b could be better along with w ~= 1 instead of 4 as mentioned in
/// [1].
///
/// [1] "Locality-sensitive hashing scheme based on p-stable distributions"
/// [2] "Hashing for Similarity Search: A Survey"
/// [3] "Coding for Random Projections"
///
/// TODO: Consider using a standard citation format. See also:
///   - https://academia.stackexchange.com/questions/41959/is-there-a-preferred-citation-style-for-computer-science-papers
impl RBP {
    // TODO: Is it too much of a burden to the caller to have the type of the
    // dimensions be Ix here, instead of something more vanilla like u8?
    fn new(vec_dim: Ix, hash_dim: Ix) -> Result<RBP> {
        let rbp = RBP {
            vec_dim,
            hash_dim,
            kernel: Array2::random(Ix2(vec_dim, hash_dim), F32(StandardNormal)),
        };
        Ok(rbp)
    }

    fn hash(&self, vecs: Array2<f32>) -> Array2<i8> {
        let mut rng = rand::thread_rng();
        let r: f32 = 4.0;
        let b = f32::from(Uniform::new_inclusive(0, r as u8).sample(&mut rng));
        vecs.dot(&self.kernel).mapv(|x| (x + b / r).floor() as i8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        for _ in 0..100 {
            let num_vecs: Ix = 2;
            let vec_dim: Ix = 3;
            let hash_dim: Ix = 8;
            let vecs = Array2::random((num_vecs, vec_dim), F32(StandardNormal));
            let rbp = RBP::new(vec_dim, hash_dim).unwrap();
            let h = rbp.hash(vecs);
            assert_eq!(h.dim(), (num_vecs, hash_dim));
        }
    }
}
