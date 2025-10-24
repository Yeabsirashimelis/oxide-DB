use liboxideDB::OxideDB;
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
    let action: &str = args.get(2).expect(&USAGE).as_ref();
    let key: &str = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = Path::new(&fname);
    let mut store = oxideDB::open(path).expect("unable to open file"); // opens the file at path
    store.load().expect("unable to load data"); //create an-in-memory index by loading the data from path

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value: &str = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "update" => {
            let value: &str = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
