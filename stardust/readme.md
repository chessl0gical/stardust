





# Stardust Keygen



**Deterministic key generator**: Turns a password into a huge, non-repeating keystream (up to 20 GiB) using Argon2id + ChaCha20.



Perfect for generating large deterministic keys, one-time pads, or testing large encryption scenarios.



## Features



- Password → deterministic keystream (same input = always identical output)

- Strong key derivation with **Argon2id** (128 MiB memory hardness)

- Fast keystream generation using **ChaCha20**

- Supports up to **20 GiB** per key file

- Cross-platform (Windows, Linux, macOS)

- Safe defaults with optional customization via CLI or environment variables

- Prevents accidental overwrites



## Installation / Building



bash

git clone <your-repo>

cd stardust-keygen

cargo build --release





The compiled binary will be at:

- Windows: `target/release/stardust-keygen.exe`

- Linux/macOS: `target/release/stardust-keygen`



## Usage



The size must be provided **in bytes**.



### Basic Examples



bash

# 1 GiB key

.stardust-keygen.exe 1073741824



# 5 GiB key

.stardust-keygen.exe 5368709120



# 10 GiB key

.stardust-keygen.exe 10737418240





### Common Sizes (copy-paste friendly)



| Desired Size     | Command (Windows)                          |

|------------------|--------------------------------------------|

| 512 MiB          | `.stardust-keygen.exe 536870912`         |

| **1 GiB**        | `.stardust-keygen.exe 1073741824`        |

| 2 GiB            | `.stardust-keygen.exe 2147483648`        |

| **5 GiB**        | `.stardust-keygen.exe 5368709120`        |

| **10 GiB**       | `.stardust-keygen.exe 10737418240`       |

| **20 GiB** (max) | `.stardust-keygen.exe 21474836480`       |



### Advanced Options



bash

# Custom output path

.stardust-keygen.exe 1073741824 --output mykey.bin



# Custom salt, pepper, or nonce

.stardust-keygen.exe 1073741824 

&#x20; --salt "my-custom-salt-2026" 

&#x20; --pepper "my-secret-pepper" 

&#x20; --nonce "ABC123XYZ789"





You can also use environment variables:



bash

# Windows PowerShell

$env:STARDUST_SALT = "my-custom-salt-2026"

$env:STARDUST_PEPPER = "my-secret-pepper"

.stardust-keygen.exe 1073741824





### Behavior



- Asks for password **twice** for confirmation

- Default output file: `key.key` (placed next to the executable)

- Will **not** overwrite an existing file (exits with error)

- Shows progress for large files

- Uses secure defaults:

&#x20; - Salt: `ultimate-salt-v1-2026`

&#x20; - Pepper: `secret-pepper-masterkey`

&#x20; - Nonce: `JohnDoeXYZ12` (exactly 12 bytes)



## Security Notes



- This tool is **deterministic**. The same password + salt + pepper + nonce will always produce the exact same key file.

- The pepper acts as a secret. Keep it safe — if it leaks, security falls back to Argon2(salt) only.

- Never reuse the same password/salt/pepper/nonce combination for different purposes.

- The generated file contains raw keystream bytes — treat it with the same care as a real cryptographic key.



## Project Details



- **Language**: Rust 2024

- **Dependencies**: clap, rpassword, argon2, chacha20, cipher

- **License**: MIT OR Apache-2.0

- **Maximum size**: 20 GiB



## Building from Source



bash

cargo build --release

# or for maximum performance:

cargo build --release --target x86_64-pc-windows-msvc   # Windows





---



**Made with ❤️ by ai **



For questions or feature requests, feel free to open an issue.





