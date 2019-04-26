use crate::distance;

use lexical;
use num_traits::ToPrimitive;
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Naive {
    vecs: HashMap<String, Vec<f32>>,
}

// TODO: Learn how to write proper rust flavored docs.
// Can we make clippy more angry for eliding these docs?
//
// TODO: Define a trait shared across different LSH methods.
impl Naive {
    pub fn from_path(path: &str, skip: usize) -> Result<Naive> {
        let mut idx = Naive {
            vecs: HashMap::new(),
        };

        let f = File::open(path)?;
        for (i, line) in BufReader::new(f).lines().enumerate() {
            if i < skip {
                continue;
            }
            let mut word: Option<String> = None;
            let mut vec = vec![];
            for (i, part) in line?.split_whitespace().enumerate() {
                if i == 0 {
                    word = Some(part.to_owned());
                    continue;
                }
                let x: f32 = lexical::try_parse(part)?;
                vec.push(x.to_owned());
            }
            match word {
                Some(word) => {
                    idx.vecs.insert(word, vec.to_owned());
                }
                None => println!("empty line: {}", i),
            }
        }

        Ok(idx)
    }

    pub fn get(&self, key: &str) -> Option<Vec<f32>> {
        match self.vecs.get(key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.vecs.len()
    }

    pub fn similar(&self, query: &[f32], k: u8) -> Vec<(String, f32)> {
        let mut heap = BinaryHeap::new();
        for (word, vec) in self.vecs.iter() {
            let d = distance::pnorm(2.0, query, vec);
            let item = (OrderedFloat(d), word);
            heap.push(item);
            if heap.len() as u8 > k {
                heap.pop();
            }
        }
        let mut sim = vec![];
        for (d, word) in heap.into_sorted_vec() {
            sim.push((word.to_owned(), d.to_f32().unwrap()));
        }
        sim
    }
}
