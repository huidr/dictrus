/// Display

use rusqlite::{Connection, Result};
use serde::Serialize;
use anyhow::Result as EResult;
use prettytable::{Table, row, cell};

/// Display word meanings
pub fn word_meanings(conn: &Connection, word: &str) -> EResult<()> {
    // Prepare the SQL query
    let mut stmt = conn.prepare(r#"
        SELECT words.word,
               synsets.definition,
               GROUP_CONCAT(samples.sample, ';') AS examples
        FROM words
        JOIN senses ON words.wordid = senses.wordid
        JOIN synsets ON senses.synsetid = synsets.synsetid
        LEFT JOIN samples ON synsets.synsetid = samples.synsetid
        WHERE words.word = ?
        GROUP BY synsets.synsetid;
        "#
    )?;
    
    // Execute query and collect results
    let rows = stmt.query_map([word], |row| {
	Ok((
	    row.get::<_, String>(0)?, // word
	    row.get::<_, String>(1)?, // definition
	    row.get::<_, Option<String>>(2)?, // examples
	))
    })?;

    // Create and format the table
    let mut table = Table::new();
    table.add_row(row![bFg => "Word", "Definition", "Examples"]);

    for row in rows {
	let (word, definition, examples) = row?;

	// Format examples with line breaks for better readability
	let examples = examples
	    .map(|e| e.replace(";", "\n"))
	    .unwrap_or_else(|| "No examples".to_string());

	table.add_row(row![word, definition, examples]);
    }

    // Print the table
    if table.len() > 1 {
	println!("\nMeanings of '{}':", word);
	table.printstd();
    } else {
	println!("No meanings found for '{}'", word);
    }

    Ok(())
}
