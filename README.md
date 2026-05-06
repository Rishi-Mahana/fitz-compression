# Fitz-Compression

A tool to readily compress and decompress files using Huffman encoding.

---

## How It Works

For encoding a file, it creates a prefix-free tree wherein each leaf represents a particular symbol occurring in the original file, arranged in order of their frequencies.

The encoded file then accords **7 bytes to metadata**:

| Bytes | Description |
|-------|-------------|
| Bytes 1–4 | Identifier for the decoder |
| Bytes 5–6 | Size (in bytes) of the sequence containing the Huffman tree |
| Byte 7 | Number of padding bits in the entire file |

These 7 bytes are followed by **N bytes** of the Huffman tree in a particular format.

The decoder discerns the information from the first **(7+N) bytes** and begins reading bitwise from there onwards. Using the reconstructed Huffman tree and the property of it being prefix-free, the decoder is able to decode the entire file in one pass.

---

## Prerequisites

- Rust
- Cargo *(included with Rust)*

---

## Installation

```bash
git clone https://github.com/Rishi-Mahana/fitz-compression.git
cd fitz-compression
```

---

## Usage

```bash
cargo run <input_file> <output_file> <encode|decode>
```

| Argument | Description |
|----------|-------------|
| `<input_file>` | File path of the file you want to read |
| `<output_file>` | File path of the file you want to write to |
| `<encode\|decode>` | Mode of operation |

---

## Examples

**Encode:**
```bash
cargo run doc.txt dec.fitz encode
```

**Decode:**
```bash
cargo run dec.fitz doc.txt decode
```
