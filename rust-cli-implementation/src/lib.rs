use std::io::{self, Read, Write};
use anyhow::Result;

pub fn process_text(input: Option<&str>, transformation: Transformation) -> Result<String> {
    let text = match input {
        Some(filename) => std::fs::read_to_string(filename)?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(match transformation {
        Transformation::Upper => text.to_uppercase(),
        Transformation::Lower => text.to_lowercase(),
        Transformation::Dedup => deduplicate_lines(&text),
    })
}

pub fn write_output(output: Option<&str>, text: &str) -> Result<()> {
    match output {
        Some(filename) => std::fs::write(filename, text)?,
        None => io::stdout().write_all(text.as_bytes())?,
    };
    Ok(())
}

fn deduplicate_lines(text: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    text.lines()
        .filter(|line| seen.insert(*line))
        .collect::<Vec<&str>>()
        .join("\n")
}

#[derive(Debug, Clone, Copy)]
pub enum Transformation {
    Upper,
    Lower,
    Dedup,
}