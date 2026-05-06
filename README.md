Fitz-Compresssion: A tool to readily to compress and decompress files using Huffman encoding. 
Working: For encoding a file, it creates a prefix-free tree wherein each leaf represents a particular symbol occuring in the original file, arranged in order of their freqeuncies. The encoded file, then accords 
7 bytes to meta data. The first 4 bytes act as an identifier for the decoder, the next 2 bytes entail the size of the sequence in bytes that contain the Huffman tree required to decode it, and the next byte entails the number of padding bits in the entire file. These 7 bytes are then followed by N bytes of the Huffman tree in a particular format.
The decoder discerns the information from the first (7+N) bytes and begins reading bitwise from there onwards. Using the reconstructed Huffman tree and he property of it being prefix-free, the decoder is able to decode the entire file in one pass.
Pre-Requisites:
i)Rust
ii)Cargo(included with Rust)
Installation:
git clone https://github.com/Rishi-Mahana/fitz-compression.git
cd fitz-compression
Usage:
cargo run <input file path> <output file path> <mode>
<input file path> -> File path of the file you want to read
<output file path> -> File path of the file you want to write to
<mode> -> "encode" or "decode"
Examples:
cargo run doc.txt dec.fitz encode
cargo run dec.fitz doc.txt decode
