use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct NtrData {
    pub data: HashMap<String, String>,
}

impl NtrData {
    pub fn new() -> Self {
        NtrData {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

pub fn parse_ntr_file<P: AsRef<Path>>(path: P) -> io::Result<NtrData> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut ntr_data = NtrData::new();
    let mut current_prefix = Vec::new();
    let mut indent_levels = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim().to_string();

        if trimmed.is_empty() || trimmed.starts_with('@') {
            current_prefix.clear();
            indent_levels.clear();
            continue;
        }

        let current_indent = line.chars().take_while(|c| c.is_whitespace()).count();

        if current_indent == 0 {
            current_prefix.clear();
            indent_levels.clear();
        }
        else {
            while !indent_levels.is_empty() && current_indent < *indent_levels.last().unwrap() {
                indent_levels.pop();
                current_prefix.pop();
            } 
        }

        if let Some(index) = trimmed.find('>') {
            let (key, value) = trimmed.split_at(index);
            let key = key.trim().to_string();
            let value = value[1..].trim().to_string();

            let full_key = if current_prefix.is_empty() {
                key
            } else {
                format!("{}.{}", current_prefix.join("."), key)
            };

            ntr_data.insert(full_key, value);
        } else {
            if indent_levels.is_empty() || current_indent > *indent_levels.last().unwrap_or(&0) {
                indent_levels.push(current_indent);
                current_prefix.push(trimmed);
            } else {
                current_prefix.pop();
                current_prefix.push(trimmed);
            }
        }
    }

    Ok(ntr_data)
}

