use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use rpassword::prompt_password;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20::ChaCha20;
use cipher::{KeyIvInit, StreamCipher};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB
const MAX_SIZE: u64 = 20 * 1024 * 1024 * 1024; // 20 GiB

#[derive(Parser)]
#[command(
    name = "stardust-keygen",
    version,
    about = "Deterministic key generator: password → non-repeating keystream up to 20 GiB",
    author = "Mark",
    long_about = None
)]
struct Cli {
    /// Key size in bytes (1 to 20 GiB)
    #[arg(index = 1)]
    size: u64,

    /// Output file path (default: key.key next to the executable)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Salt for Argon2 (default: ultimate-salt-v1-2026)
    #[arg(long, value_name = "SALT")]
    salt: Option<String>,

    /// Pepper (secret key) for Argon2 (default: secret-pepper-masterkey)
    #[arg(long, value_name = "PEPPER")]
    pepper: Option<String>,

    /// 12-byte nonce for ChaCha20 (default: JohnDoeXYZ12)
    #[arg(long, value_name = "NONCE")]
    nonce: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.size == 0 || cli.size > MAX_SIZE {
        eprintln!("Error: Size must be between 1 and 20 GiB ({} bytes).", MAX_SIZE);
        std::process::exit(1);
    }

    // Determine output path
    let output_path = cli.output.unwrap_or_else(|| {
        let mut path = env::current_exe().expect("Failed to get executable path");
        path.pop(); // remove executable name
        path.push("key.key");
        path
    });

    if output_path.exists() {
        eprintln!("Error: Output file {:?} already exists. Will not overwrite.", output_path);
        std::process::exit(1);
    }

    // Config with precedence: CLI > ENV > default
    let salt = cli.salt
        .or_else(|| env::var("STARDUST_SALT").ok())
        .unwrap_or_else(|| "ultimate-salt-v1-2026".into())
        .into_bytes();

    let pepper = cli.pepper
        .or_else(|| env::var("STARDUST_PEPPER").ok())
        .unwrap_or_else(|| "secret-pepper-masterkey".into())
        .into_bytes();

    let nonce_str = cli.nonce
        .or_else(|| env::var("STARDUST_NONCE").ok())
        .unwrap_or_else(|| "JohnDoeXYZ12".into());

    if nonce_str.len() != 12 {
        eprintln!("Error: Nonce must be exactly 12 bytes/characters.");
        std::process::exit(1);
    }
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(nonce_str.as_bytes());

    // Password input
    let pw1 = prompt_password("Enter password: ")?;
    let pw2 = prompt_password("Confirm password: ")?;
    if pw1 != pw2 {
        eprintln!("Error: Passwords do not match!");
        std::process::exit(1);
    }
    let password = pw1.into_bytes();

    // Argon2id key derivation (master key)
    let mut master_key = [0u8; 32];
    let params = Params::new(131_072, 4, 8, None)
        .expect("Invalid Argon2 parameters");

    let argon2 = Argon2::new_with_secret(
        &pepper,
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    ).expect("Failed to initialize Argon2");

    argon2
        .hash_password_into(&password, &salt, &mut master_key)
        .expect("Argon2 hashing failed");

    // ChaCha20 setup - clean, non-deprecated way (fixes your compile error)
    let mut cipher = ChaCha20::new(&master_key.into(), &nonce.into());

    // Generate keystream in chunks
    let mut output = File::create(&output_path)?;
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut remaining = cli.size;
    let total = cli.size;

    println!("Generating {} bytes (~{:.2} GiB) of deterministic key material...", 
             total, total as f64 / 1024.0 / 1024.0 / 1024.0);

    while remaining > 0 {
        let chunk = std::cmp::min(CHUNK_SIZE as u64, remaining) as usize;

        cipher.apply_keystream(&mut buffer[..chunk]);
        output.write_all(&buffer[..chunk])?;

        remaining -= chunk as u64;

        // Simple progress feedback
        if remaining % (50 * CHUNK_SIZE as u64) == 0 || remaining == 0 {
            let done = total - remaining;
            let percent = (done as f64 / total as f64) * 100.0;
            print!("\rProgress: {:.1}% ({:.2} / {:.2} GiB)   ", 
                   percent, 
                   done as f64 / 1024.0 / 1024.0 / 1024.0,
                   total as f64 / 1024.0 / 1024.0 / 1024.0);
            let _ = io::stdout().flush();
        }
    }

    println!("\n\nKey file successfully created: {:?}", output_path);
    Ok(())
}