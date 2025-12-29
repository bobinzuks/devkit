use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

/// Show current time as Unix timestamp
pub fn now() -> Result<String> {
    let now = Utc::now();
    let local = Local::now();
    
    Ok(format!(
        "Unix timestamp: {}\n\
         UTC:   {}\n\
         Local: {}",
        now.timestamp(),
        now.format("%Y-%m-%d %H:%M:%S UTC"),
        local.format("%Y-%m-%d %H:%M:%S %Z")
    ))
}

/// Convert Unix timestamp to human-readable
pub fn from_unix(timestamp: i64) -> Result<String> {
    // Handle both seconds and milliseconds
    let (secs, millis) = if timestamp > 10_000_000_000 {
        // Likely milliseconds
        (timestamp / 1000, timestamp % 1000)
    } else {
        (timestamp, 0)
    };

    let dt = DateTime::from_timestamp(secs, (millis * 1_000_000) as u32)
        .context("Invalid timestamp")?;
    
    let local: DateTime<Local> = dt.into();
    
    Ok(format!(
        "UTC:   {}\n\
         Local: {}\n\
         ISO:   {}",
        dt.format("%Y-%m-%d %H:%M:%S UTC"),
        local.format("%Y-%m-%d %H:%M:%S %Z"),
        dt.to_rfc3339()
    ))
}

/// Convert human-readable date to Unix timestamp
pub fn to_unix(date: &str) -> Result<String> {
    // Try various formats
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d",
        "%d-%m-%Y %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
    ];

    for fmt in &formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(date, fmt) {
            let dt = Utc.from_utc_datetime(&naive);
            return Ok(format!(
                "Unix timestamp: {}\n\
                 Milliseconds:   {}",
                dt.timestamp(),
                dt.timestamp_millis()
            ));
        }
        
        // Try date-only formats
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date, fmt) {
            let naive = naive_date.and_hms_opt(0, 0, 0).unwrap();
            let dt = Utc.from_utc_datetime(&naive);
            return Ok(format!(
                "Unix timestamp: {}\n\
                 Milliseconds:   {}",
                dt.timestamp(),
                dt.timestamp_millis()
            ));
        }
    }

    // Try ISO 8601
    if let Ok(dt) = DateTime::parse_from_rfc3339(date) {
        return Ok(format!(
            "Unix timestamp: {}\n\
             Milliseconds:   {}",
            dt.timestamp(),
            dt.timestamp_millis()
        ));
    }

    anyhow::bail!(
        "Could not parse date. Try formats like:\n\
         • 2024-12-28 15:30:00\n\
         • 2024-12-28\n\
         • 2024-12-28T15:30:00Z (ISO 8601)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let result = now().unwrap();
        assert!(result.contains("Unix timestamp"));
        assert!(result.contains("UTC"));
    }

    #[test]
    fn test_from_unix() {
        let result = from_unix(0).unwrap();
        assert!(result.contains("1970-01-01"));
    }

    #[test]
    fn test_from_unix_millis() {
        // Test that milliseconds are handled
        let result = from_unix(1703808000000).unwrap();
        assert!(result.contains("2023"));
    }

    #[test]
    fn test_to_unix() {
        let result = to_unix("2024-01-01 00:00:00").unwrap();
        assert!(result.contains("1704067200"));
    }
}
