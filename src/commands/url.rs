use anyhow::Result;

/// URL encode a string
pub fn encode(input: &str) -> Result<String> {
    let encoded: String = input
        .chars()
        .map(|c| {
            match c {
                // Unreserved characters (RFC 3986)
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                    c.to_string()
                }
                // Everything else gets percent-encoded
                _ => {
                    let mut buf = [0; 4];
                    let bytes = c.encode_utf8(&mut buf);
                    bytes
                        .bytes()
                        .map(|b| format!("%{:02X}", b))
                        .collect::<String>()
                }
            }
        })
        .collect();
    
    Ok(encoded)
}

/// URL decode a string
pub fn decode(input: &str) -> Result<String> {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            // Collect hex digits
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    // Handle multi-byte UTF-8 sequences
                    let mut bytes = vec![byte];
                    
                    // Check for continuation bytes
                    while chars.peek() == Some(&'%') {
                        let pos = chars.clone().skip(1).take(2).collect::<String>();
                        if let Ok(next_byte) = u8::from_str_radix(&pos, 16) {
                            if next_byte & 0xC0 == 0x80 {
                                // It's a continuation byte
                                chars.next(); // consume '%'
                                chars.next(); // consume first hex
                                chars.next(); // consume second hex
                                bytes.push(next_byte);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    
                    if let Ok(s) = String::from_utf8(bytes) {
                        result.push_str(&s);
                    } else {
                        result.push('%');
                        result.push_str(&hex);
                    }
                } else {
                    result.push('%');
                    result.push_str(&hex);
                }
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            // Handle + as space (common in query strings)
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple() {
        assert_eq!(encode("hello world").unwrap(), "hello%20world");
    }

    #[test]
    fn test_encode_special() {
        assert_eq!(encode("a=b&c=d").unwrap(), "a%3Db%26c%3Dd");
    }

    #[test]
    fn test_decode_simple() {
        assert_eq!(decode("hello%20world").unwrap(), "hello world");
    }

    #[test]
    fn test_decode_plus() {
        assert_eq!(decode("hello+world").unwrap(), "hello world");
    }

    #[test]
    fn test_roundtrip() {
        let original = "Hello World! @#$%^&*()";
        let encoded = encode(original).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_unicode() {
        let original = "Hello 🚀 World";
        let encoded = encode(original).unwrap();
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }
}
