use rusqlite::{Connection, Result};
use serde::Serialize;
use anyhow::Result as EResult;

#[derive(Debug, Serialize)]
struct MyRow {
    title: String,
    author: String,
}

fn main() -> EResult<()> {
    let conn = Connection::open("/home/ronald/Documents/mylibrary.db")?;

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
