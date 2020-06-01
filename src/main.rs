use clap::Clap;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::iter::Iterator;

#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "tacogips")]
struct Opts {
    #[clap(short, long)]
    dest_file: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

struct SencenceDataFile<'a> {
    file_path: &'a str,
    sentence_presence: SentenceCountsData,
}

impl<'a> SencenceDataFile<'a> {
    fn load_or_new(file_path: &'a String) -> SencenceDataFile<'a> {
        let dest_file = File::open(file_path);

        let sentence_presence = match dest_file {
            Ok(file) => {
                let counts_file: serde_json::Result<SentenceCountsData> =
                    serde_json::from_reader(file);
                counts_file.unwrap_or_else(|e| panic!("parse dest file error {}", e))
            }
            Err(_) => SentenceCountsData {
                counts: HashMap::new(),
            },
        };

        SencenceDataFile {
            file_path: file_path.as_str(),
            sentence_presence,
        }
    }

    fn write(&mut self) {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.file_path)
            .unwrap();

        serde_json::to_writer(f, &self.sentence_presence)
            .unwrap_or_else(|e| panic!("failed to write file {} {}", self.file_path, e));
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SentenceCountsData {
    #[serde(flatten)]
    counts: HashMap<String, i32>,
}

impl SentenceCountsData {
    fn add_sentence(&mut self, sentence: String) {
        *self.counts.entry(sentence).or_insert(0) += 1;
    }

    fn sorted_vec(&self, rev: bool) -> Vec<(&String, &i32)> {
        let mut a: Vec<(&String, &i32)> = self.counts.iter().collect();

        if rev {
            a.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
        } else {
            a.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
        }
        a
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

fn add(add_opt: Add, dest_file: &mut SencenceDataFile) {
    dest_file.sentence_presence.add_sentence(add_opt.sentence);
    dest_file.write()
}

fn show(show_opt: Show, dest_file: &mut SencenceDataFile) {
    for (each_word, n) in dest_file.sentence_presence.sorted_vec(show_opt.reverse) {
        if show_opt.verbose {
            println!("{} {}", each_word, n);
        } else {
            println!("{}", each_word);
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let dest_file = &mut SencenceDataFile::load_or_new(&opts.dest_file);
    match opts.subcmd {
        SubCommand::Add(add_opt) => add(add_opt, dest_file),
        SubCommand::Show(show_opt) => show(show_opt, dest_file),
    }
}
