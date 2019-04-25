use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use lexical;
use num_traits::ToPrimitive;
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod distance;
mod hash;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct NaiveIndex {
    vecs: HashMap<String, Vec<f32>>,
}

// TODO: Learn how to write proper rust flavored docs.
// Can we make clippy more angry for eliding these docs?
//
// TODO: Define a trait shared across different LSH methods.
impl NaiveIndex {
    fn from_path(path: &str, skip: usize) -> Result<NaiveIndex> {
        let mut idx = NaiveIndex {
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

    fn similar(&self, query: &[f32], k: u8) -> Vec<(String, f32)> {
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

fn main() -> Result<()> {
    let matches = App::new("blush")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("query")
                .about("query for nearest neighbors given some vec key")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("data").required(true))
                .arg(Arg::with_name("index").long("index").default_value("naive")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("query") {
        let key = matches.value_of("key").unwrap();
        let data = matches.value_of("data").unwrap();
        match matches.value_of("index") {
            Some("naive") => {
                // TODO: We should really have a separate parser type. For instance glove,
                // fasttext, generic text embedding file, or binary embedding file.
                let idx = NaiveIndex::from_path(data, 1)?;
                dbg!(idx.vecs.len());
                let vec = &idx.vecs[key];
                dbg!(idx.similar(vec, 8));
            }
            // TODO: What's a better way to return CLI errors here?
            Some(index) => println!("unsupported index: {}", index),
            None => unreachable!(),
        }
    }
    Ok(())
}
