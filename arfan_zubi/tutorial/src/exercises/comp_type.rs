pub struct CompoundTypes;
impl CompoundTypes {
    // String is a compound type because it is made up of char types.
    pub fn exercise_001() {
        let s: &str = "hello, world";
        println!("Success!\ns: '{}'", s);
    }
    pub fn exercise_002() {
        let s: Box<str> = "hello, world".into();
        greetings(&s);
        let s: &str = "hello, world";
        greetings(s);

        fn greetings(s: &str) {
            println!("s: '{}'", s)
        }
    }
    pub fn exercise_003() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');
        assert_eq!(s, "hello, world!");
        println!("Success!\ns: '{}'", s);
    }
    pub fn exercise_004() {
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        // s += &"!".to_string();
        s += "!";
        println!("s: '{}'", s);
    }
    pub fn exercise_005() {
        let s = String::from("I like dogs");
        //  Allocae new memory and store the modified string there.
        let s1 = s.replace("dogs", "cats");
        assert_eq!(s1, "I like cats");
        println!("Success!\ns: '{}'\ns1: '{}'", s, s1);
    }
    pub fn exercise_006() {
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        // let s3 = s1.clone() + s2.as_str();
        assert_eq!(s3, "hello,world!");
        println!("s1: '{}'\ns2: '{}'\ns3: '{}'", s1, s2, s3);
    }
    pub fn exercise_007() {
        let s = "hello, world";
        greetings(s.to_string());
        greetings(String::from(s));
        greetings(s.to_owned());

        fn greetings(s: String) {
            println!("s: '{}'", s)
        }
    }
    pub fn exercise_008() {
        let s = "hello, world".to_string();
        let s1: &str = &s;
        println!("Success!\ns: '{}'\ns1: '{}'", s, s1);
        let s1: &str = s.as_str();
        println!("s: '{}'\ns1: '{}'", s, s1);
    }
    pub fn exercise_009() {
        // You can use escapes to writ bytes by their hexadecimal values;
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape) ;
        // Or Unicode points.
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
        println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);
        let long_string = "String literals 
                                can span multiple lines. 
                                The linebreak and indentation here \
                                 can be escaped too!";
        println!("{}", long_string);
    }
    pub fn exercise_010() {
        // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        let raw_str = "Escapes don't work here: \x3F \u{211D}";
        assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("quotes: {}", quotes);
        // You can use up to 65535 #s.
        let delimiter = r###"A string with "# in it. And even "##!"###;
        println!("delimiter: {}", delimiter);
        let long_delimiter= r###"Hello, "##""###;
        assert_eq!(long_delimiter, "Hello, \"##\"");
        println!("long_delimiter: {}", long_delimiter);
    }
    pub fn exercise_011() {
        // You can't use index to access a char in a string, but you can in a slice.
        // 中国 chinese zhōng guó for middle country.
        let s1 = String::from("hi,中国");
        let h = &s1[0..1];
        assert_eq!(h, "h");
        let h1 = &s1[3..6];
        assert_eq!(h1, "中");
        println!("Success!\ns1: '{}' meaning hi,middle country (zhōng guó)\nh: '{}'\nh1: '{}'", s1, h, h1);
    }
    pub fn exercise_012() {
        // 你好, 世界 chinese Nǐ hăo, shìjiè for Hello, world.
        for c in "你好, 世界".chars() {
            print!("{}", c)
        }
        println!("\n你好, 世界 is chinese Nǐ hăo, shìjiè for Hello, world.");
    }
}
