/// An offline CLI dictionary
use clap::Parser;
use rusqlite::Connection;
use anyhow::Result as AResult;
use std::env;

pub mod lib;
use lib::display_meanings;

/// Fetches meaning(s) of a word with examples and grammatical categories
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// The word whose meaning you want to search
    word: String,

    /// Maximum number of examples for each meaning
    #[arg(short, long, default_value_t = false)]
    examples: bool
}

fn main() -> AResult<()> {
    let args = Args::parse();
    let word = args.word;
    let examples = args.examples;

    // Path to WordNet SQLite file
    let path = env::var("HOME").expect("Home directory not found")
	+ "/.config/dictrus/wordnet.sqlite";
    
    // Open the WordNet SQLite database
    let conn = Connection::open(&path)?;

    // Query and display meanings
    match examples {
	false => lib::display_meanings(&conn, &word)?,
	true => lib::display_meanings_with_examples(&conn, &word)?,
    }

    Ok(())
}
