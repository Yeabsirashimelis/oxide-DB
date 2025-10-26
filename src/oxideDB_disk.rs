use lib_oxide_db::OxideDB;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    oxideDB_disk.exe FILE get KEY
    oxideDB_disk.exe FILE delete KEY
    oxideDB_disk.exe FILE insert KEY VALUE
    oxideDB_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
   oxideDB_disk FILE get KEY
   oxideDB_disk FILE delete KEY
   oxideDB_disk FILE insert KEY VALUE
   oxideDB_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

pub fn store_index_on_disk(db: &mut OxideDB, index_key: &ByteStr) {
    db.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&db.index).unwrap();
    db.index = std::collections::HashMap::new();
    db.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    pub const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut db = OxideDB::open(path).expect("unable to open file");

    db.load2().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = db.get(&INDEX_KEY).unwrap().unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);

            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = db.get_at(i).unwrap();
                    let kv_string = String::from_utf8(kv.value.clone());
                    match kv_string {
                        Ok(value_str)=> println!("{}", value_str),
                        Err(_)=> println!("data is not convertible to string. so you will only get the bytes, use them appropriately")
                    }
                    println!("{:?}", kv.value)
                }
            }
        }

        "delete" => db.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            db.insert(key, value).unwrap();
            store_index_on_disk(&mut db, INDEX_KEY);
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            db.update(key, value).unwrap();
            store_index_on_disk(&mut db, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    }
}
