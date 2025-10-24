/*
   ByteStr is to &str what ByteString is to Vec<u8>.

   This code processes lots of Vec<u8> data. Because that is used in the same way as String tends to be used,
    ByteString is a useful alias.
*/

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Seek, SeekFrom},
    path::Path,
};

use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;

type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct OxideDB {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl OxideDB {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        let index = HashMap::new();
        Ok(OxideDB { f, index })
    }

    // OxideDB::load() - populates the index of the OxideDB struct, mapping keys to file positions
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
            // File::seek() returns the number of bytes from the start of the file. This becomes the value of the index
            let position = f.seek(SeekFrom::Current(0))?;

            // OxideDB::process_record - reads a record in the file at its current position
            let maybe_kv = OxideDB::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    /*
                    File operations in Rust might return an error of type std::io::ErrorKind::UnexpectedEoF

                    */
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, position);
        }
        Ok(())
    }
}
