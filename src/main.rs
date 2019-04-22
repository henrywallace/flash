use num_traits::ToPrimitive;
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn norm(p: f32, u: &[f32], v: &[f32]) -> f32 {
    if u.len() != v.len() {
        panic!("unequal vector lengths {} != {}", u.len(), v.len());
    }
    let mut dot = 0.0;
    for (x, y) in u.iter().zip(v) {
        dot += (x - y).powf(p)
    }
    dot.powf(1.0 / p)
}

struct NaiveIndex {
    vecs: HashMap<String, Vec<f32>>,
}

// TODO: Learn how to write proper rust flavored docs.
// Can we make clippy more angry for eliding these docs?
//
// TODO: Define a trait shared across different LSH methods.
impl NaiveIndex {
    fn new() -> NaiveIndex {
        NaiveIndex {
            vecs: HashMap::new(),
        }
    }

    fn load(&mut self, path: &str, skip: usize) {
        let f = File::open(path).unwrap();
        for (i, line) in BufReader::new(f).lines().enumerate() {
            if i < skip {
                continue;
            }
            let mut word: Option<String> = None;
            let mut vec = vec![];
            for (i, part) in line.unwrap().split_whitespace().enumerate() {
                if i == 0 {
                    word = Some(part.to_owned());
                    continue;
                }
                let x: f32 = part.parse().unwrap();
                vec.push(x.to_owned());
            }
            match word {
                Some(word) => {
                    self.vecs.insert(word, vec.to_owned());
                }
                None => println!("empty line: {}", i),
            }
        }
    }

    fn similar(&self, query: &[f32], k: u8) -> Vec<(String, f32)> {
        let mut heap = BinaryHeap::new();
        for (word, vec) in self.vecs.iter() {
            let d = norm(2.0, query, vec);
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

fn main() {
    let mut n = NaiveIndex::new();
    // TODO: Create script to download prepare this data.
    // See https://fasttext.cc/docs/en/english-vectors.html
    // Here we head -10000 for sake of faster manual testing.
    n.load("/home/henrywallace/Downloads/cc.en.300.10k.vec", 1);
    dbg!(n.vecs.len());
    let vec = &n.vecs["president"];
    dbg!(n.similar(vec, 8));
}
