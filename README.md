# Text Analyzer (Rust)

A high-performance command-line tool designed to perform frequency analysis on text files. This project demonstrates idiomatic Rust patterns, focusing on **Functional Programming**.

---

##  Project Structure

- `main.rs`: Orchestrates data flow and handles top-level error propagation.
- `config.rs`: Parses CLI arguments using functional chains.
- `analyzer.rs`: The core engine utilizing an iterator-based pipeline to process text.
- `stats.rs`: Manages data aggregation and exports results to JSON/CSV formats.

---



##  Usage Guide & Examples

| Feature        | Terminal Command                                                |
|----------------|----------------------------------------------------------------|
| Basic Run      | `cargo run -- input.txt`                                        |
| Min Length     | `cargo run -- input.txt --min-length 6`                        |
| Starts With    | `cargo run -- input.txt --starts-with b`                       |
| No Stop Words  | `cargo run -- input.txt --no-stop-word`                        |
| JSON Export    | `cargo run -- input.txt --output results.json --format json`   |
| CSV Export     | `cargo run -- input.txt --output results.csv --format csv`     |
| Combined       | `cargo run -- input.txt --min-length 5 --no-stop-word --output out.json` |


### Installation
Ensure you have Rust installed, then build the project:
```bash
cargo build --release
