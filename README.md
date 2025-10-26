OxideDB
OxideDB — A Rust-based, Riak-inspired key–value store with on-disk indexing.
A lightweight single-file database mimicking Riak’s Bitcask engine, featuring append-only writes, CRC32 checksum validation, in-memory and optional persisted indexes for faster startup and efficient key lookups, designed for reliable and compact key–value storage.

Features
Key–Value Storage: Stores arbitrary byte arrays (Vec<u8>) as keys and values.
Append-Only Writes: Data is appended to the file, preventing in-place overwrites.
In-Memory Index (v1): oxideDB_mem builds the index on load by scanning the file.
Persisted On-Disk Index (v2): oxideDB_disk stores the index under a special +index key for faster startup.
CRC32 Checksums: Ensures data integrity when reading records from disk.
Single-File Database: Compact storage with simple file-based access.
Automatic File Creation: The database file is automatically created if it does not exist — no manual setup required.

Project Structure
oxide-DB/
├── Cargo.toml
├── src/
│   ├── lib.rs         # Core OxideDB library
│   ├── oxideDB_mem.rs # Binary for in-memory index (v1)
│   └── oxideDB_disk.rs# Binary for on-disk persisted index (v2)


lib_oxide_db: Contains all core database logic (insert, update, delete, get, index management).
oxideDB_mem: Version 1 executable; keeps the index in memory only.
oxideDB_disk: Version 2 executable; stores the index on disk under +index.

Installation

Clone the repository:
git clone https://github.com/yourusername/oxide-DB.git
cd oxide-DB

Build the binaries:
cargo build --release

You will find the executables in target/release/:
oxideDB_mem → memory-only index
oxideDB_disk → persisted on-disk index
The database file will be automatically created if it does not exist; no manual file creation is necessary.

Usage
Insert a key-value pair:
./oxideDB_disk data.db insert myKey myValue

Retrieve a value by key:
./oxideDB_disk data.db get myKey

Update a value:
./oxideDB_disk data.db update myKey newValue

Delete a key:
./oxideDB_disk data.db delete myKey

Replace oxideDB_disk with oxideDB_mem to use the memory-only version.


How It Works
1, File Storage: Each record consists of a CRC32 checksum, key length, value length, and the raw key-value bytes.
2, Indexing:
   - v1 (oxideDB_mem): Scans the file on load and builds an in-memory HashMap of key → file offset.
   - v2 (oxideDB_disk): Stores the serialized index on disk as a special +index record for faster startup.
3, Data Integrity: Checksums verify that stored data is not corrupted.
4, Insert/Update/Delete: Always append-only; updates create new records, maintaining historical data.
5, Automatic File Handling: The database file is automatically created when needed; users do not have to pre-create it.

Comparison to Riak

OxideDB is conceptually inspired by Riak’s Bitcask storage engine:
Key–Value Model: Both store arbitrary keys and values.
Append-Only Storage: Writes are append-only to simplify crash recovery.
In-Memory Index: Provides fast lookups without scanning the entire file.

Differences:
OxideDB is a single-file, local store; Riak is distributed with replication and clustering.
OxideDB focuses on simplicity and efficiency for local storage; Riak is production-grade with advanced features like vector clocks and eventual consistency.
License

## Author

Yeabsira Shimelis

  Built with Rust 🦀
