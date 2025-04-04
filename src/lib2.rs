use rusqlite::{Connection, Result};
use anyhow::Result as EResult;

pub fn word_meanings(conn: &Connection, word: &str) -> EResult<()> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            ss.posid,
            ss.definition,
            GROUP_CONCAT(DISTINCT sm.sample, '; ') as examples,
            (
                SELECT GROUP_CONCAT(DISTINCT w2.word, ', ')
                FROM senses s2
                JOIN words w2 ON s2.wordid = w2.wordid
                WHERE s2.synsetid = ss.synsetid AND w2.word != ?
                ORDER BY w2.word
            ) as synonyms,
            (
                SELECT GROUP_CONCAT(DISTINCT w3.word, ', ')
                FROM semrelations sr
                JOIN relations r ON sr.relationid = r.relationid
                JOIN senses s3 ON sr.synset2id = s3.synsetid
                JOIN words w3 ON s3.wordid = w3.wordid
                WHERE sr.synset1id = ss.synsetid AND r.name = 'antonym'
                ORDER BY w3.word
            ) as antonyms
        FROM words w
        JOIN senses s ON w.wordid = s.wordid
        JOIN synsets ss ON s.synsetid = ss.synsetid
        LEFT JOIN samples sm ON ss.synsetid = sm.synsetid
        WHERE w.word = ?
        GROUP BY ss.synsetid
        ORDER BY ss.posid, s.senseid;
        "#
    )?;

    println!("\nMeanings of '{}':", word);

    let rows = stmt.query_map([word, word], |row| {
        Ok((
            row.get::<_, i64>(0)?,  // posid
            row.get::<_, String>(1)?,  // definition
            row.get::<_, Option<String>>(2)?,  // examples
            row.get::<_, Option<String>>(3)?,  // synonyms
            row.get::<_, Option<String>>(4)?,  // antonyms
        ))
    })?;

    for row in rows {
        let (posid, definition, examples, synonyms, antonyms) = row?;
        
        // Map numeric posid to POS abbreviation
        let pos_symbol = match posid {
            1 => "(n)",    // noun
            2 => "(v)",    // verb
            3 => "(adj)",  // adjective
            4 => "(adv)",  // adverb
            5 => "(adj)",  // adjective satellite (usually treated as adjective)
            _ => "(?)",
        };
        println!("{} {}", pos_symbol, definition);

        // Print examples if they exist
        if let Some(examples_str) = examples {
            for example in examples_str.split("; ") {
                if !example.trim().is_empty() {
                    println!("      \"{}\"", example.trim().trim_matches('"'));
                }
            }
        }

        // Print synonyms if they exist
        if let Some(syns) = synonyms {
            if !syns.is_empty() {
                println!("      Synonyms: {}", syns);
            }
        }

        // Print antonyms if they exist
        if let Some(ants) = antonyms {
            if !ants.is_empty() {
                println!("      Antonyms: {}", ants);
            }
        }

        // Add spacing between meanings
        println!();
    }

    Ok(())
}

