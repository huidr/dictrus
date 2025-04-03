use clap::Parser;
use anyhow::Result as EResult;
use rusqlite::Connection;

pub mod display;
use display::word_meanings;

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
    display::word_meanings(&conn, &word)?;

    Ok(())
}


/*
#[derive(Debug, Serialize)]
struct MyRow {
    title: String,
    author: String,
}

fn main() -> EResult<()> {
    let conn = Connection::open("~/.config/qdict/wordnet.sqlite")?;

    let mut stmt = conn.prepare(
	"SELECT title, author FROM everymans_library_fiction"
    )?;

    let rows = stmt.query_map([], |row| {
	Ok(MyRow {
	    title: row.get(0)?,
	    author: row.get(1)?,
	})
    });

    let rows = rows?;

    for row_result in rows {
	let row = row_result?;
	println!("{:#?}", row)
    }

    Ok(())
}
*/
