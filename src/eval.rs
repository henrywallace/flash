use crate::index::Index;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Eval {
    queries: Vec<String>,
}

// Goals of Eval:
//   - Load query keys from file.
//     In the future we could instead have queries be read vectors, or even generated from some
//     distribution. But for now, we simply use keys read from a file.
//  - Start with index to get perfect gold similar.

#[derive(Debug, Serialize, Deserialize)]
pub struct GoldNeighbors {
    // TODO: Include the metric used for these neighbors. Which requires a proper type or name in
    // the first place.
    key: String,
    neighbors: Vec<(String, f32)>,
}

impl Eval {
    pub fn from_path(path: &str) -> Result<Eval> {
        let f = File::open(path)?;
        let queries = BufReader::new(f)
            .lines()
            .collect::<std::result::Result<_, _>>()?;
        Ok(Eval { queries })
    }

    pub fn gen<I: Index>(&self, idx: I) -> Result<Vec<GoldNeighbors>> {
        let mut gold = vec![];
        for key in self.queries.iter() {
            let vec = idx.get(key).ok_or_else(|| format!("{} missing", key))?;
            let neighbors = idx.similar(vec.as_slice(), 8);
            gold.push(GoldNeighbors {
                key: key.clone(),
                neighbors,
            })
        }
        Ok(gold)
    }
}
