use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::convert::TryInto;

use clap::Parser;
use rpassword::prompt_password;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20::ChaCha20;
use cipher::{KeyIvInit, StreamCipher};
use sha2::{Digest, Sha256};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB
const MAX_SIZE: u64 = 20 * 1024 * 1024 * 1024; // 20 GiB

#[derive(Parser)]
#[command(
    name = "stardust-keygen",
    version,
    about = "Deterministic key generator: password → reproducible keystream",
)]
struct Cli {
    /// Key size in bytes (1 to 20 GiB)
    #[arg(index = 1)]
    size: u64,

    /// Optional context (e.g. "github", "backup-2026")
    #[arg(short, long)]
    context: Option<String>,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.size == 0 || cli.size > MAX_SIZE {
        eprintln!("Error: Size must be between 1 and 20 GiB.");
        std::process::exit(1);
    }

    // Determine output path
    let output_path = cli.output.unwrap_or_else(|| {
        let mut path = env::current_exe().expect("Failed to get executable path");
        path.pop();
        path.push("key.key");
        path
    });

    if output_path.exists() {
        eprintln!("Error: Output file already exists.");
        std::process::exit(1);
    }

    // ========================
    // PASSWORD INPUT
    // ========================
    let pw1 = prompt_password("Enter password: ")?;
    let pw2 = prompt_password("Confirm password: ")?;
    if pw1 != pw2 {
        eprintln!("Error: Passwords do not match.");
        std::process::exit(1);
    }

    let mut password = pw1.into_bytes();

    // ========================
    // CONTEXT (acts like salt)
    // ========================
    let context = cli.context.unwrap_or_else(|| "default".into());

    // Combine password + context
    let mut input = password.clone();
    input.extend_from_slice(context.as_bytes());

    // ========================
    // ARGON2 → MASTER KEY
    // ========================
    let mut master_key = [0u8; 32];

    let params = Params::new(
        131_072, // 128 MB memory
        3,       // iterations
        4,       // parallelism
        None,
    ).expect("Invalid Argon2 params");

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    // Deterministic salt derived from context
    let salt = Sha256::digest(format!("stardust-salt:{}", context));

    argon2
        .hash_password_into(&input, &salt, &mut master_key)
        .expect("Argon2 failed");

    // ========================
    // DOMAIN SEPARATION
    // ========================

    // stream_key = SHA256(master_key || "stream")
    let mut hasher = Sha256::new();
    hasher.update(master_key);
    hasher.update(b"stream");
    let stream_key = hasher.finalize();

    // nonce = first 12 bytes of SHA256(master_key || "nonce")
    let mut hasher = Sha256::new();
    hasher.update(master_key);
    hasher.update(b"nonce");
    let nonce_hash = hasher.finalize();

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&nonce_hash[..12]);

    // Convert stream_key to fixed-size array
    let key: [u8; 32] = stream_key[..32]
        .try_into()
        .expect("Invalid key length");

    // ========================
    // CHACHA20 SETUP
    // ========================
    let mut cipher = ChaCha20::new(
        &key.into(),
        &nonce.into(),
    );

    // ========================
    // STREAM GENERATION
    // ========================
    let mut output = File::create(&output_path)?;
    let mut buffer = vec![0u8; CHUNK_SIZE];

    let mut remaining = cli.size;
    let total = cli.size;

    println!(
        "Generating {} bytes (~{:.2} GiB)...",
        total,
        total as f64 / 1024.0 / 1024.0 / 1024.0
    );

    while remaining > 0 {
        let chunk = std::cmp::min(CHUNK_SIZE as u64, remaining) as usize;

        cipher.apply_keystream(&mut buffer[..chunk]);
        output.write_all(&buffer[..chunk])?;

        remaining -= chunk as u64;

        let done = total - remaining;

        if done % (50 * CHUNK_SIZE as u64) == 0 || remaining == 0 {
            let percent = (done as f64 / total as f64) * 100.0;
            print!(
                "\rProgress: {:.1}% ({:.2} / {:.2} GiB)",
                percent,
                done as f64 / 1024.0 / 1024.0 / 1024.0,
                total as f64 / 1024.0 / 1024.0 / 1024.0
            );
            let _ = io::stdout().flush();
        }
    }

    println!("\n\nDone: {:?}", output_path);

    // ========================
    // CLEANUP (basic memory wipe)
    // ========================
    password.fill(0);
    master_key.fill(0);

    Ok(())
}