/*
   ByteStr is to &str what ByteString is to Vec<u8>.

   This code processes lots of Vec<u8> data. Because that is used in the same way as String tends to be used,
    ByteString is a useful alias.
*/

use byteorder::{ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use byteorder::LittleEndian;
use crc::crc32;
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct OxideDB {
    pub f: File,
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
            // .seek is a method that moves the file cursor (the current read/write position).
            // SeekFrom::Current(0) means : move the cursor 0 vytes from its current position
            //   so this call doesnot move the cursro- it just returns the current byte position in the file
            let current_position = f.seek(SeekFrom::Current(0))?;

            // OxideDB::process_record - reads a record in the file at its current position
            let maybe_kv = OxideDB::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    /*
                    File operations in Rust might return an error of type std::io::ErrorKind::UnexpectedEoF, EOF(end of file) is a
                     convention that operating system provide to applications. there is no special marker or delimeter at the end of a file
                     within the file itself.

                     EoF is a zero byte(0u8). when reading from a file, the OS tells the application how many bytes were successfully read from storage.
                      if no bytes were successfully read from disk, yet no error condition was detecte, then the OS and, therefore, the application can
                       assume the EOF has been reached.

                    */
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(kv.key, current_position);
        }
        Ok(())
    }

    /*
       the process_record() function does the processing for this within OxideDB. it begins with reading 12 bytes that represent 3 integers:
          - a checksum
          - the length of the key
          - the length of the value
    */
    //      f may be any type that implements Read, such as a type that reads files, but can also be &[u8]
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        // the byteorder crate allows on-disk integers to be read in a deterministic manner.
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let val_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            // f.by_ref() is required b/c take(n) creates a new Read value. Using a reference within this short-lived block sidesteps ownership issues.
            // This creates a “limited reader” that will stop reading after data_len bytes.
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = OxideDB::process_record(&mut f)?;
        Ok(kv)
    }
}
