# OxideDB

RUST + OXYGEN = OXIDE

A lightweight, Riak-inspired key–value store written in Rust. OxideDB supports append-only writes, CRC32 checksum validation, in-memory and persisted on-disk indexes, and automatic file creation. Designed for reliable, compact key–value storage in a single-file database.

# Features

- Key–value storage using arbitrary byte arrays (Vec<u8>)
- Append-only writes for safe updates and crash recovery
- CRC32 checksum validation for data integrity
- In-memory index (v1) via oxideDB_mem
- Persisted on-disk index (v2) via oxideDB_disk using the special +index key
- Automatic database file creation if it does not exist
- Efficient startup and lookups with optional persisted index

# Project Structure
```
src/
├── lib.rs           # Core OxideDB library (insert, update, delete, get, index management)
├── oxideDB_mem.rs   # Binary for in-memory index version (v1)
└── oxideDB_disk.rs  # Binary for persisted on-disk index version (v2)
```

# How it works
1, Each record in the file contains: a CRC32 checksum, key length, value length, and the raw key-value bytes.
2, Indexing:
   - v1 (oxideDB_mem): Scans the file on load to build an in-memory HashMap of key → file offset.
   - v2 (oxideDB_disk): Saves the serialized index to disk under the +index key for faster startup.
3, Insert/Update/Delete: Always append-only; updates create new records, preserving historical data.

4, Automatic file handling: The database file is automatically created if it does not exist.

# Run locally
Clone the repository and build with Cargo:
```powershell
git clone https://github.com/yourusername/oxide-DB.git
cd oxide-DB
cargo build --release
```

Executables are located in target/release/:
   oxideDB_mem → memory-only index (v1)
   oxideDB_disk → persisted on-disk index (v2)

# Usage Examples
Using oxideDB_mem (v1 — in-memory index)

Insert a key-value pair:
```
./oxideDB_mem data.db insert myKey myValue
```
Retrieve a value:
```
./oxideDB_mem data.db get myKey
```
Update a value:
```
./oxideDB_mem data.db update myKey newValue
```
Delete a key:
```
./oxideDB_mem data.db delete myKey
```

Using oxideDB_disk (v2 — persisted on-disk index)
Insert a key-value pair:
```
./oxideDB_disk data.db insert myKey myValue
```

Retrieve a value:
```
./oxideDB_disk data.db get myKey
```

Update a value:
```
./oxideDB_disk data.db update myKey newValue
```

Delete a key:
```
./oxideDB_disk data.db delete myKey
```

oxideDB_disk automatically updates the on-disk index for faster startup, while oxideDB_mem rebuilds the index in memory each time.

# Comparison to Riak
OxideDB is inspired by Riak’s Bitcask engine:
   - Similarities: Key–value model, append-only writes, in-memory index for fast lookups.
   - Differences: OxideDB is a single-file local store, not distributed. Riak is production-grade, supporting clustering, replication, and eventual consistency.

# Author

Yeabsira Shimelis

Built with Rust 🦀