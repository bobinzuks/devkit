use anyhow::{bail, Context, Result};
use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

/// Generate MD5 hash
pub fn md5(input: &str) -> Result<String> {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Generate SHA256 hash
pub fn sha256(input: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Generate SHA512 hash
pub fn sha512(input: &str) -> Result<String> {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Hash a file with specified algorithm
pub fn hash_file(path: &str, algorithm: &str) -> Result<String> {
    let bytes = fs::read(path)
        .with_context(|| format!("Failed to read file: {}", path))?;

    let hash = match algorithm.to_lowercase().as_str() {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(&bytes);
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(&bytes);
            format!("{:x}", hasher.finalize())
        }
        _ => bail!("Unknown algorithm: {}. Use md5, sha256, or sha512.", algorithm),
    };

    Ok(format!("{} ({}) = {}", algorithm.to_uppercase(), path, hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        // Known MD5 hash of "hello"
        assert_eq!(md5("hello").unwrap(), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha256() {
        // Known SHA256 hash of "hello"
        assert_eq!(
            sha256("hello").unwrap(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha512() {
        let result = sha512("hello").unwrap();
        assert_eq!(result.len(), 128); // SHA512 produces 128 hex characters
    }
}
