use lib_oxide_db::OxideDB;
use std::path::Path;

/*
   the cfg attribute allows windows users to see the correct file extension in their help documentation.

   here there is a conditional compilation. what is compiled depending on the compiler target arichtecture
*/
#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    oxideDB_mem.exe FILE get KEY
    oxideDB_mem.exe FILE delete KEY
    oxideDB_mem.exe FILE insert KEY VALUE
    oxideDB_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    oxideDB_mem FILE get KEY
    oxideDB_mem FILE delete KEY
    oxideDB_mem FILE insert KEY VALUE
    oxideDB_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE);
    let maybe_value = args.get(4);


    let path = Path::new(&fname);
    let mut store = OxideDB::open(path).expect("unable to open file"); // opens the file at path
    store.load().expect("unable to load data"); //create an-in-memory index by loading the data from path

    match action {
            // as_bytes() is a string-specific method that converts a text string (&str) into its raw byte representation (&[u8])
        "get" => match store.get(key.as_bytes()).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key.as_bytes()).unwrap(),
        "insert" => {
            let value: &str = maybe_value.expect(&USAGE).as_ref();
            store.insert(key.as_bytes(), value.as_bytes()).unwrap()
        }
        "update" => {
            let value: &str = maybe_value.expect(&USAGE).as_ref();
            store.update(key.as_bytes(), value.as_bytes()).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
