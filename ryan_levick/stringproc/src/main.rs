use std::collections::HashMap;

fn main() {
    // 1)
    let path = "Links_u16.txt";
    let contents = std::fs::read(path).unwrap();
    let contents: Vec<u16> = contents
        .chunks(2)
        .map(|bytes| {
            let [first, second] = [bytes[0], bytes[1]];
            (first as u16) << 8 | (second as u16)
        })
        .collect();
    let contents = String::from_utf16(&contents[..]).unwrap();
    println!("{}", &contents);
    println!("{:?}", process(&contents).into_keys());
}

fn process<'a>(string: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in string.lines() {
        for sentence in line.split_terminator('.') {
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    result.entry(word).or_default().push(sentence);
                }
            }
        }
    }
    result
}

/* 
1)
    Rust expects strings to be utf-8. std::fs::read_to_string will panic!() if not utf-8. 
    std::fs::read returns bytes instead. If file is in u16 format, then try to get 2 chunks() at a 
    time.
*/