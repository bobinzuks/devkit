use anyhow::{bail, Result};
use uuid::Uuid;

/// Generate UUIDs
pub fn generate(count: u32, version: u8, uppercase: bool) -> Result<String> {
    let mut results = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let uuid = match version {
            4 => Uuid::new_v4(),
            7 => Uuid::now_v7(),
            _ => bail!("Unsupported UUID version: {}. Use 4 or 7.", version),
        };

        let uuid_str = if uppercase {
            uuid.to_string().to_uppercase()
        } else {
            uuid.to_string()
        };

        results.push(uuid_str);
    }

    Ok(results.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_single_v4() {
        let result = generate(1, 4, false).unwrap();
        assert_eq!(result.len(), 36); // UUID length with hyphens
    }

    #[test]
    fn test_generate_multiple() {
        let result = generate(5, 4, false).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_uppercase() {
        let result = generate(1, 4, true).unwrap();
        assert_eq!(result, result.to_uppercase());
    }

    #[test]
    fn test_invalid_version() {
        let result = generate(1, 5, false);
        assert!(result.is_err());
    }
}
