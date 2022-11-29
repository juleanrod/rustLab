use std::collections::HashMap;

fn main() {
    let skip = std::env::args().skip(1);
    let mut arguments = skip;
    let _key = arguments.next().unwrap();
    // The following is a custom way to print if the value was there or not
    //let _key = arguments.next().expect("Key was not there.");
    let _value = arguments.next().unwrap();
    println!("The key: {}, the value: {}", _key, _value);
    let contents = format!("{}\t{}\n", _key, _value);

    // to read about the fs module go to -> docs.rs/std
    let write_result = std::fs::write("kv.db", contents);

    // pattern matching -> similar to a switch statement
    //match write_result {
    //Ok(()) => {

    //}
    //Err(e) => {

    //}
    //}
    let detabase = Database::new().expect("Database::new() crashed");
}

// structure then implement the structure
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // read the kv.db file
        //let contents = match std::fs::read_to_string("kv.db") {
        //Ok(c) => c,
        //Err(error) => {
        //return Result::Err(error);
        //}
        //}
        let contents = std::fs::read_to_string("kv.db")?;
        // parse the string
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            // populate our map
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: HashMap::new(),
        })
    }
}
