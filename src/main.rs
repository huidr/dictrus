use anyhow::Result as EResult;
use rusqlite::Connection;

pub mod lib2;
use lib2::word_meanings;

/*
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
 */

fn main() -> EResult<()> {
    let conn = Connection::open("/home/ronald/.config/qdict/wordnet.sqlite")?;
    
    // Verify the database schema
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;
    println!("Available tables: {:?}", tables);
    
    // Query word meanings
    lib2::word_meanings(&conn, "love")?;
    Ok(())
}

