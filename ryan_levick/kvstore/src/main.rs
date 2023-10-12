use std::collections::HashMap;

fn main() {
    // 1)
    let mut arguments = std::env::args().skip(1);
    // 2)
    let key = arguments.next().expect("Key was not there");
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    // 3)
    // 4)
    // 5)
    let mut database = Database::new().expect("Creating db failed");
    // 13)
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    // 20)
    match database.flush() {
        Ok(()) => println!("YAY"),
        Err(err) => println!("OH NO! Error! {}", err)
    }
}

struct Database {
    map: HashMap<String, String>,
    // 20)
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // 6)
        let mut map = HashMap::new();
        // 7)
        let contents = std::fs::read_to_string("kv.db")?;
        // 8)
        for line in contents.lines() {
            // 9)
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            // 10)
            map.insert(key.to_owned(), value.to_owned());
        }
        // 11)
        Ok(Database {
            map,
            // 20)
            flush: false
        })
    }
    // 12)
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    // 14)
    fn flush(mut self) -> std::io::Result<()> {
        // 20)
        self.flush = true;
        // 18)
        do_flush(&self)
    }
}

// 17)
impl Drop for Database {
    fn drop(&mut self) {
        // 18)
        // 19)
        // 20)
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

// 18)
fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("Do flush called!");
    let mut contents = String::new();
    for (key, value) in &database.map {
        // 16)
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    // 15)
    std::fs::write("kv.db", contents)
}

/* 
20)
    To keep track of if flush has been run and not have it run again in the trait Drop.
    database.flush().unwrap();
19)
    Ignore the posibility of returned error.
18)
    Moved to a stand alone function.
    let mut contents = String::new();
    for (key, value) in &self.map {
        // 16)
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    // 15)
    std::fs::write("kv.db", contents)
17)
    To not have to call flush everytime add a trait that runs a function when database is about to
    go out of scope.
16)
    To make the following more efficient and not create a temporary string that only goes out of 
    scope and is not moved.
    let kvpair = format!("{}\t{}\n", key, value);
    contents.push_str(&kvpair);
15)
    todo!("Finish this method") prevents the compiler from highlighting every error above and
    is a placeholder to allow completing the code without error highlighting. Says to compiler 
    just ignore type checking the rest of the function it is within.
14)
    Shorthand for Result<(), std::io::Error>.
    When borrowing something you can't take ownership of parts of it.
13)
    Database is moved into the insert function and now self of that method owns that 
    memmory. Self then gets dropped when it goes out of scope in the end of that method. Same with
    key and value. to_uppercase() only borrows. println!() macro borrows the values of variables 
    only if the value has not been moved.
12) 
    Methods take self as their 1st argument and functions (fn) don't. mut self takes ownership,
    where as &mut self borrows. In order of more demanding to less: &mut self or &self, mut self or 
    self.
11)
    populate the map. When the field of a struct and the variable have the same name you only have
    to write it once.
10)
    to_owned() copies the value in memory and without you only have a pointer &str which is a 
    diferent type to String.
9)
    parse the string.
    Tutorial on older version of rust without split_once() as core function.
    let mut chunks = line.splitn(2, '\t'); let key = chunks.next().expect("No Key!")
    let value = chunks.next().expect("No value!").
8)
    &str are views into owned strings. line is a pointer (fat pointer) to individual lines.
    String is a struct with: location on heap, length of string, capacity (size can grow to).
    &String is a borrowing of String with the 3 above, &str just has location and length.
7)
    Variables are bindings. ? bubbles up to caller if there is an error.
6)
    Read the kv.db file. 
    let contents = match std::fs::read_to_string("kv.db") {
        Ok(contents) => c,
        Err(error) => {
            return Err(error);
        }
    };
    Shorthand for the above.String
5)
    .expect() will crash the programme on error. Without it the programme will run as returning an
    error does not crash the programmme.
    let database = Database::new().expect("Database::new() crashed.");
4)
    Returns a result type, like success or error
    let write_result = std::fs::write("kv.db", contents);
    // Use pattern matching (switch) to catch errors and success. Pattern matching is an expression
    // not a statment and can be bound to, to get the returned result.
    match write_result {
        Ok(()) => println!("Write Succeeded!"),
        Err(e) => println!("Write Failed! \nCause: {}", e),
    }
    Or
    match std::fs::write("kv.db", contents) {
        Ok(()) => println!("Write Succeeded!"),
        Err(e) => println!("Write Failed! \nCause: {}", e),
    }
    Now replaced by let mut database = ....
    std::fs::write("kv.db", contents).unwrap();
3)
    format! macro returns a string. Macros can take a variable number of arguments but functions 
    can't.
    Now replaced by let mut database = ....
    let contents = format!("{}\t{}\n", key, value);
2)
    View a detailed error description by running: cargo check
    Inferred Option<String> is rust's way of saying it could be null (none existant)
    .unwrap() on a None value will crash (panic()) the programme
    To crash with .unwrap(): cargo run, to not crash: cargo run -- hello
    Instead of unwrap use .expect("The message to user").
1)
    ::args is an iterator. You can skip over items.
    Cannot write let mut arguments: Args = because Args is not yet in scope.
About:
    rust is a low level programming language. It is a statically typed language. It will also
    infer our types. Types are usually structs. Does not have a garbage collector constantly 
    keeping track of how long a value is used. Has ownership instead. Variable will go out of scope 
    at the end of the function and then the value will be freed from memory. Can transfer ownership 
    between bindings. There are no keyword arguments and only positional arguments.
*/