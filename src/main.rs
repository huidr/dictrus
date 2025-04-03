use clap::Parser;
use anyhow::Result as EResult;
use rusqlite::Connection;

pub mod lib;
use lib::word_meanings;

#[derive(Parser, Debug)]
struct Args {
    word: String,
}

fn main() -> EResult<()> {
    let args = Args::parse();
    let word = args.word;
    
    // Open the WordNet SQLite database
    let conn = Connection::open("/home/ronald/.config/qdict/wordnet.sqlite")?;

    // Query and display meanings
    lib::word_meanings(&conn, &word)?;

    Ok(())
}
