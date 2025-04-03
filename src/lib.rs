use rusqlite::{Connection, Result};
use textwrap::{indent, Options};
// use serde::Serialize;
use anyhow::Result as EResult;
// use prettytable::{Table, row, cell};

pub fn word_meanings(conn: &Connection, word: &str) -> EResult<()> {
    // Modified query to include part of speech
    let mut stmt = conn.prepare(
        r#"
        SELECT ld.posid, ss.definition, GROUP_CONCAT(sm.sample, '; ') as examples
        FROM words w
        JOIN senses s ON w.wordid = s.wordid
        JOIN synsets ss ON s.synsetid = ss.synsetid
        JOIN domains ld ON ss.domainid = ld.domainid
        LEFT JOIN samples sm ON ss.synsetid = sm.synsetid
        WHERE w.word = ?
        GROUP BY ss.synsetid
        ORDER BY ld.posid, s.senseid;
        "#
    )?;

    println!("\nMeanings of '{}':", word);

    let rows = stmt.query_map([word], |row| {
        Ok((
            row.get::<_, String>(0)?,  // pos
            row.get::<_, String>(1)?,  // definition
            row.get::<_, Option<String>>(2)?,  // examples
        ))
    })?;

    // Configure text wrapping
    let wrap_options = Options::new(70)
        .initial_indent("      ")
        .subsequent_indent("      ");

    for row in rows {
        let (pos, definition, examples) = row?;
        
        // Print part of speech and definition
        let pos_symbol = match pos.as_str() {
            "n" => "(n)",
            "v" => "(v)",
            "a" | "s" => "(adj)", // 'a' for adjective, 's' for satellite adjective
            "r" => "(adv)",
            _ => "(?)",
        };
        println!("{} {}", pos_symbol, definition);

        // Print examples if they exist
        if let Some(examples_str) = examples {
            let examples_list: Vec<&str> = examples_str.split("; ").collect();
            for example in examples_list {
                if !example.is_empty() {
                    let cleaned_example = example.trim_matches('"');
                    println!("      \"{}\"", cleaned_example);
                }
            }
        }
    }

    Ok(())
}

// Example usage:
// display_word_meanings(&conn, "love")?;
