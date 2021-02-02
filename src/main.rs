use anyhow::{anyhow, Error, Result};
use clap::Clap;
use fslock::LockFile;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Clap, Debug)]
#[clap(version = "0.2.2", author = "tacogips")]
struct Opts {
    #[clap(short, long)]
    dest_file: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Serialize, Deserialize, Debug)]
struct SentenceCountsData {
    #[serde(flatten)]
    counts: HashMap<String, i32>,
}

impl SentenceCountsData {
    fn new() -> SentenceCountsData {
        Self {
            counts: HashMap::new(),
        }
    }
    fn load(sentences_file_path: &str) -> Result<SentenceCountsData> {
        let mut sentence_sentence_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(sentences_file_path)?;

        let mut str_contents = String::new();
        sentence_sentence_file.read_to_string(&mut str_contents)?;

        let sentence_counts_data: serde_json::Result<SentenceCountsData> =
            serde_json::from_str(&str_contents);
        sentence_counts_data.or_else(|_| {
            if str_contents.is_empty() {
                Ok(Self::new())
            } else {
                Err(anyhow!(
                    "{} is invalid sentence counter file",
                    sentences_file_path
                ))
            }
        })
    }

    fn add_sentence(&mut self, sentence: String) {
        *self.counts.entry(sentence).or_insert(0) += 1;
    }

    fn sorted_vec(&self, rev: bool) -> Vec<(&String, &i32)> {
        let mut result: Vec<(&String, &i32)> = self.counts.iter().collect();

        if rev {
            result.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
        } else {
            result.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
        }
        result
    }
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(about = "Increments number of appearance of the word")]
    Add(Add),
    #[clap(about = "Show words sorted by its number of appearance")]
    Show(Show),
}

#[derive(Clap, Debug)]
struct Add {
    sentence: String,
}

// Show all words
#[derive(Clap, Debug)]
struct Show {
    #[clap(
        short,
        long,
        about = "Show in descending order of the number of appearance"
    )]
    reverse: bool,
    #[clap(short, long, about = "Show number of appearance")]
    verbose: bool,
}

fn add(add_opt: Add, data: &mut SentenceCountsData, dest_file_path: &str) -> Result<()> {
    let dest_sentence_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(dest_file_path)?;

    data.add_sentence(add_opt.sentence);
    serde_json::to_writer(dest_sentence_file, data).map_err(Error::msg)
}

fn show(show_opt: Show, data: &SentenceCountsData) -> Result<()> {
    for (each_word, n) in data.sorted_vec(show_opt.reverse) {
        if show_opt.verbose {
            println!("{} {}", each_word, n);
        } else {
            println!("{}", each_word);
        }
    }
    Ok(())
}
//
fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let lock_file_path = format!("{}.lock", opts.dest_file);
    let mut _locked_dest_file = LockFile::open(&lock_file_path)?;

    let mut sentence_data = SentenceCountsData::load(&opts.dest_file)?;

    match opts.subcmd {
        SubCommand::Add(add_opt) => add(add_opt, &mut sentence_data, &opts.dest_file),
        SubCommand::Show(show_opt) => show(show_opt, &mut sentence_data),
    }
}
