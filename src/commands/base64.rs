use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use std::fs;

/// Encode a string to Base64
pub fn encode(input: &str) -> Result<String> {
    let encoded = general_purpose::STANDARD.encode(input.as_bytes());
    Ok(encoded)
}

/// Decode a Base64 string
pub fn decode(input: &str) -> Result<String> {
    let bytes = general_purpose::STANDARD
        .decode(input.trim())
        .context("Invalid Base64 input")?;
    
    let decoded = String::from_utf8(bytes)
        .context("Decoded bytes are not valid UTF-8")?;
    
    Ok(decoded)
}

/// Encode a file to Base64
pub fn encode_file(path: &str) -> Result<String> {
    let bytes = fs::read(path)
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let encoded = general_purpose::STANDARD.encode(&bytes);
    Ok(encoded)
}

/// Decode Base64 to a file
pub fn decode_file(input: &str, output: &str) -> Result<String> {
    let bytes = general_purpose::STANDARD
        .decode(input.trim())
        .context("Invalid Base64 input")?;
    
    fs::write(output, &bytes)
        .with_context(|| format!("Failed to write file: {}", output))?;
    
    Ok(format!("✓ Decoded {} bytes to {}", bytes.len(), output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello").unwrap(), "aGVsbG8=");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("aGVsbG8=").unwrap(), "hello");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Hello, World! 🚀";
        let encoded = encode(original).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
