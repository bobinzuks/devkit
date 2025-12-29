use clap::{Parser, Subcommand};
use colored::*;

mod commands;

#[derive(Parser)]
#[command(name = "devkit")]
#[command(author = "DevKit Tools")]
#[command(version = "1.0.0")]
#[command(about = "⚡ Blazing fast offline developer utilities", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🔐 Base64 encode/decode strings and files
    #[command(arg_required_else_help = true)]
    Base64 {
        #[command(subcommand)]
        action: Base64Action,
    },

    /// 🆔 Generate UUIDs (v4 or v7)
    Uuid {
        /// Number of UUIDs to generate
        #[arg(short, long, default_value = "1")]
        count: u32,

        /// UUID version (4 or 7)
        #[arg(short, long, default_value = "4")]
        version: u8,

        /// Output in uppercase
        #[arg(short = 'U', long)]
        uppercase: bool,
    },

    /// #️⃣ Generate hash checksums (MD5, SHA256, SHA512)
    Hash {
        #[command(subcommand)]
        action: HashAction,
    },

    /// 📋 JSON formatting and validation
    #[command(arg_required_else_help = true)]
    Json {
        #[command(subcommand)]
        action: JsonAction,
    },

    /// ⏰ Unix timestamp conversions
    #[command(arg_required_else_help = true)]
    Time {
        #[command(subcommand)]
        action: TimeAction,
    },

    /// 🔗 URL encode/decode strings
    #[command(arg_required_else_help = true)]
    Url {
        #[command(subcommand)]
        action: UrlAction,
    },
}

#[derive(Subcommand)]
enum Base64Action {
    /// Encode a string to Base64
    Encode {
        /// The string to encode
        input: String,
    },
    /// Decode a Base64 string
    Decode {
        /// The Base64 string to decode
        input: String,
    },
    /// Encode a file to Base64
    EncodeFile {
        /// Path to the file
        path: String,
    },
    /// Decode Base64 to a file
    DecodeFile {
        /// Base64 input
        input: String,
        /// Output file path
        output: String,
    },
}

#[derive(Subcommand)]
enum HashAction {
    /// Generate MD5 hash
    Md5 {
        /// Input string to hash
        input: String,
    },
    /// Generate SHA256 hash
    Sha256 {
        /// Input string to hash
        input: String,
    },
    /// Generate SHA512 hash
    Sha512 {
        /// Input string to hash
        input: String,
    },
    /// Hash a file
    File {
        /// Path to the file
        path: String,
        /// Algorithm: md5, sha256, sha512
        #[arg(short, long, default_value = "sha256")]
        algorithm: String,
    },
}

#[derive(Subcommand)]
enum JsonAction {
    /// Format/prettify JSON
    Format {
        /// JSON string or file path
        input: String,
        /// Indentation spaces
        #[arg(short, long, default_value = "2")]
        indent: usize,
    },
    /// Minify JSON (remove whitespace)
    Minify {
        /// JSON string or file path
        input: String,
    },
    /// Validate JSON syntax
    Validate {
        /// JSON string or file path
        input: String,
    },
}

#[derive(Subcommand)]
enum TimeAction {
    /// Show current time as Unix timestamp
    Now,
    /// Convert Unix timestamp to human-readable
    FromUnix {
        /// Unix timestamp (seconds)
        timestamp: i64,
    },
    /// Convert human-readable date to Unix timestamp
    ToUnix {
        /// Date string (e.g., "2024-12-28 12:00:00")
        date: String,
    },
}

#[derive(Subcommand)]
enum UrlAction {
    /// URL encode a string
    Encode {
        /// String to encode
        input: String,
    },
    /// URL decode a string
    Decode {
        /// URL-encoded string to decode
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Base64 { action } => match action {
            Base64Action::Encode { input } => commands::base64::encode(&input),
            Base64Action::Decode { input } => commands::base64::decode(&input),
            Base64Action::EncodeFile { path } => commands::base64::encode_file(&path),
            Base64Action::DecodeFile { input, output } => {
                commands::base64::decode_file(&input, &output)
            }
        },

        Commands::Uuid {
            count,
            version,
            uppercase,
        } => commands::uuid::generate(count, version, uppercase),

        Commands::Hash { action } => match action {
            HashAction::Md5 { input } => commands::hash::md5(&input),
            HashAction::Sha256 { input } => commands::hash::sha256(&input),
            HashAction::Sha512 { input } => commands::hash::sha512(&input),
            HashAction::File { path, algorithm } => commands::hash::hash_file(&path, &algorithm),
        },

        Commands::Json { action } => match action {
            JsonAction::Format { input, indent } => commands::json::format(&input, indent),
            JsonAction::Minify { input } => commands::json::minify(&input),
            JsonAction::Validate { input } => commands::json::validate(&input),
        },

        Commands::Time { action } => match action {
            TimeAction::Now => commands::time::now(),
            TimeAction::FromUnix { timestamp } => commands::time::from_unix(timestamp),
            TimeAction::ToUnix { date } => commands::time::to_unix(&date),
        },

        Commands::Url { action } => match action {
            UrlAction::Encode { input } => commands::url::encode(&input),
            UrlAction::Decode { input } => commands::url::decode(&input),
        },
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}
