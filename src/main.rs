use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use std::error::Error;

mod distance;
mod hash;
mod index;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
                let idx = index::Naive::from_path(data, 1)?;
                dbg!(idx.len());
                let vec = idx.get(key).unwrap();
                dbg!(idx.similar(vec.as_slice(), 8));
            }
            // TODO: What's a better way to return CLI errors here?
            Some(index) => println!("unsupported index: {}", index),
            None => unreachable!(),
        }
    }
    Ok(())
}
