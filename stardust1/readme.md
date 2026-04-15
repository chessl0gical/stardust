# Stardust Keygen 1 🔑✨

**Deterministic key generator: password → reproducible keystream (1 byte to 20 GiB)**

---

## 🚀 Overview

**Stardust Keygen** is a deterministic cryptographic tool that transforms a password (plus optional context) into a **reproducible, high-entropy keystream** of arbitrary size.

* Same password + context → **identical output every time**
* No stored metadata required
* No hidden constants or baked-in secrets
* Works across machines and environments

---

## 🔐 How It Works

Stardust follows a clean cryptographic pipeline:

```
password + context
        ↓
Argon2id (strong KDF)
        ↓
master key (32 bytes)
        ↓
domain separation (SHA-256)
   ├─ stream key (32 bytes)
   └─ nonce (12 bytes)
        ↓
ChaCha20 stream cipher
        ↓
deterministic keystream output
```

---

## ✨ Features

* 🔁 **Deterministic** – regenerate identical keys anytime
* 🔒 **Secure primitives** – Argon2id + ChaCha20
* 🧠 **Context-based separation** – like “namespaces” for keys
* 📦 **No file headers or metadata needed**
* ⚡ **Scales up to 20 GiB**
* 💻 **Cross-platform reproducibility**

---

## 📦 Installation

### 1. Clone the repo

```bash
git clone https://github.com/yourname/stardust-keygen.git
cd stardust-keygen
```

### 2. Build

```bash
cargo build --release
```

Binary will be in:

```
target/release/stardust-keygen
```

---

## 🧪 Usage

### Basic

```bash
stardust-keygen <SIZE_IN_BYTES>
```

Example (1 GiB):

```bash
stardust-keygen 1073741824
```

---

### With context (recommended)

```bash
stardust-keygen <SIZE> --context <LABEL>
```

Example:

```bash
stardust-keygen 1G --context github
stardust-keygen 1G --context banking
```

➡ Same password, different context = completely different keys

---

### Output file

By default:

```
key.key
```

Custom output:

```bash
stardust-keygen 1G --output mykey.bin
```

---

## 🧠 Key Concepts

### 🔑 Determinism

Stardust does **not store anything**.

To regenerate a key, you must provide:

* the **same password**
* the **same context**
* the **same size**

---

### 🧩 Context = Domain Separation

Think of `--context` like a namespace:

| Context  | Result       |
| -------- | ------------ |
| `github` | Unique key A |
| `email`  | Unique key B |
| `backup` | Unique key C |

Even with the same password, outputs are unrelated.

---

### 🔒 Security Model

Security depends on:

* Strength of your **password**
* Proper use of **context separation**

What Stardust guarantees:

* No nonce reuse issues
* No hidden backdoors
* No reliance on compiled-in secrets
* Memory-hard password hashing (Argon2id)

---

## ⚠️ Important Notes

### ❗ Not a full encryption tool

This generates **raw keystream data**, not encrypted files.

Do NOT:

* treat output as encrypted data
* reuse the same stream for multiple encryptions

---

### 🔁 Reuse warning

Using the same:

```
password + context
```

twice produces identical output.

That’s by design — but you must manage usage carefully.

---

### 🔐 Password strength matters

Weak passwords = weak security.

Use:

* long passphrases
* password managers if needed

---

## 💡 Use Cases

* Deterministic key generation
* Reproducible cryptographic material
* One-time pads (advanced users only)
* Backup key regeneration
* Experimental crypto workflows

---

## 🛠 Dependencies

* `argon2` – password hashing
* `chacha20` – stream cipher
* `sha2` – hashing (domain separation)
* `clap` – CLI parsing
* `rpassword` – secure password input

---

## 🔮 Future Ideas

* File format with verification hash
* Multi-key derivation (subkeys)
* Authenticated encryption mode
* GPU-resistant parameter tuning
* Library version (API)

---

## 📜 License

MIT OR Apache-2.0

---

## ⚡ Final Thought

Stardust is built on a simple idea:

> **A password can deterministically generate an entire universe of cryptographic data.**

Use it carefully. 🔥
