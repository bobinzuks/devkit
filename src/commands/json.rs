use anyhow::{Context, Result};
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Read input from string or file
fn read_input(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        fs::read_to_string(input)
            .with_context(|| format!("Failed to read file: {}", input))
    } else {
        Ok(input.to_string())
    }
}

/// Format/prettify JSON
pub fn format(input: &str, _indent: usize) -> Result<String> {
    let content = read_input(input)?;
    
    let value: Value = serde_json::from_str(&content)
        .context("Invalid JSON syntax")?;
    
    // Use serde_json's built-in pretty printing
    serde_json::to_string_pretty(&value).context("Failed to format JSON")
}

/// Minify JSON (remove all whitespace)
pub fn minify(input: &str) -> Result<String> {
    let content = read_input(input)?;
    
    let value: Value = serde_json::from_str(&content)
        .context("Invalid JSON syntax")?;
    
    serde_json::to_string(&value).context("Failed to minify JSON")
}

/// Validate JSON syntax
pub fn validate(input: &str) -> Result<String> {
    let content = read_input(input)?;
    
    match serde_json::from_str::<Value>(&content) {
        Ok(value) => {
            let obj_count = count_objects(&value);
            let arr_count = count_arrays(&value);
            let key_count = count_keys(&value);
            
            Ok(format!(
                "{} JSON is valid!\n  {} objects\n  {} arrays\n  {} keys",
                "✓".green().bold(),
                obj_count,
                arr_count,
                key_count
            ))
        }
        Err(e) => {
            Ok(format!(
                "{} Invalid JSON at line {}, column {}\n  {}",
                "✗".red().bold(),
                e.line(),
                e.column(),
                e
            ))
        }
    }
}

fn count_objects(value: &Value) -> usize {
    match value {
        Value::Object(map) => {
            1 + map.values().map(count_objects).sum::<usize>()
        }
        Value::Array(arr) => arr.iter().map(count_objects).sum(),
        _ => 0,
    }
}

fn count_arrays(value: &Value) -> usize {
    match value {
        Value::Object(map) => map.values().map(count_arrays).sum(),
        Value::Array(arr) => 1 + arr.iter().map(count_arrays).sum::<usize>(),
        _ => 0,
    }
}

fn count_keys(value: &Value) -> usize {
    match value {
        Value::Object(map) => {
            map.len() + map.values().map(count_keys).sum::<usize>()
        }
        Value::Array(arr) => arr.iter().map(count_keys).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let input = r#"{"name":"test","value":123}"#;
        let result = format(input, 2).unwrap();
        assert!(result.contains('\n'));
        assert!(result.contains("name"));
    }

    #[test]
    fn test_minify() {
        let input = r#"{
            "name": "test",
            "value": 123
        }"#;
        let result = minify(input).unwrap();
        assert!(!result.contains('\n'));
        assert!(!result.contains("  "));
    }

    #[test]
    fn test_validate_valid() {
        let input = r#"{"valid": true}"#;
        let result = validate(input).unwrap();
        assert!(result.contains("valid"));
    }

    #[test]
    fn test_validate_invalid() {
        let input = r#"{"invalid": }"#;
        let result = validate(input).unwrap();
        assert!(result.contains("Invalid"));
    }
}
