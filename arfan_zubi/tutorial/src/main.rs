use std::{io::{stdin, stdout, Write}, fs, path::Path, env};
// mod exercises_original;
// use exercises_original::*;
mod exercises;
use exercises::*;

enum Value {
    Int(isize),
    Float(f64),
    Bool(bool),
}

use Value::*;

struct Command {
    is_empty: bool,
    is_numeric: bool,
    value: String,
}
impl Command {
    fn new(value: String) -> Self {
        match Command::parse_string(&value) {
            Some(Int(i)) => Self {
                is_empty: false,
                is_numeric: true,
                value: i.to_string(),
            },
            Some(Float(f)) => Self {
                is_empty: false,
                is_numeric: true,
                value: f.to_string(),
            },
            Some(Bool(b)) => {
                Self {
                    is_empty: b,
                    is_numeric: false,
                    value: String::new(),
                }
            },
            None => Self {
                is_empty: true,
                is_numeric: false,
                value: String::new(),
            }
        }
    }
    fn parse_string(s: &str) -> Option<Value> {
        if let Ok(i) = s.parse() {
            Some(Int(i))
        } else if let Ok(f) = s.parse() {
            Some(Float(f))
        } else {
            Some(Bool(s.is_empty()))
        }
    }
}

fn main() {
    let args = if env::args().len() > 1 {
        env::args().skip(1).collect::<Vec<String>>()[0].to_string()
    } else {
        "".to_string()
    };
    let interact = Command::new(args);
    let file_path = format!("{}", std::env::current_dir().unwrap().to_owned().display());
    let source = fs::read_to_string(Path::new(&file_path).join("src/exercises_original.rs"))
        .ok().unwrap_or("".to_string());
    let msg = if interact.is_empty {""} else {"\n(press enter only to quit)"};
    let prompt = format!(
        "\nPlease enter a number between 1 and {} for the exercise to run{}: ",
        source.matches("pub fn ex").count(),
        msg
    );
    loop {
        let mut input = String::new();
        if !interact.is_numeric {
            print!("\n{}", prompt);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            stdout().flush().unwrap();
            if input.trim().is_empty() {
                break
            }
        } else {
            input = interact.value.to_string();
        }
        println!("\nRunning exercise_{:0>3}: \n", &*input.trim());
        match &*input.trim() {
            "1" => Variables::exercise_001(),
            "2" => Variables::exercise_002(),
            "3" => Variables::exercise_003(),
            "4" => Variables::exercise_004(),
            "5" => Variables::exercise_005(),
            "6" => Variables::exercise_006(),
            "7" => Variables::exercise_007(),
            "8" => Variables::exercise_008(),
            "9" => Variables::exercise_009(),
            "10" => Integers::exercise_001(),
            "11" => Integers::exercise_002(),
            "12" => Integers::exercise_003(),
            "13" => Integers::exercise_004(),
            "14" => Integers::exercise_005(),
            "15" => Integers::exercise_006(),
            "16" => FloatingPoint::exercise_001(),
            "17" => FloatingPoint::exercise_002(),
            "18" => FloatingPoint::exercise_003(),
            "19" => FloatingPoint::exercise_004(),
            "20" => Computations::exercise_001(),
            "21" => CharBoolUnit::exercise_001(),
            "22" => CharBoolUnit::exercise_002(),
            "23" => CharBoolUnit::exercise_003(),
            "24" => CharBoolUnit::exercise_004(),
            "25" => CharBoolUnit::exercise_005(),
            "26" => CharBoolUnit::exercise_006(),
            "27" => ExpressionStatement::exercise_001(),
            "28" => ExpressionStatement::exercise_002(),
            "29" => ExpressionStatement::exercise_003(),
            "30" => ExpressionStatement::exercise_004(),
            "31" => Functions::exercise_001(),
            "32" => Functions::exercise_002(),
            "33" => Functions::exercise_003(),
            "34" => Functions::exercise_004(),
            "35" => Functions::exercise_005(),
            "36" => Ownership::exercise_001(),
            "37" => Ownership::exercise_002(),
            "38" => Ownership::exercise_003(),
            "39" => Ownership::exercise_004(),
            "40" => Ownership::exercise_005(),
            "41" => Ownership::exercise_006(),
            "42" => Ownership::exercise_007(),
            "43" => Ownership::exercise_008(),
            "44" => Ownership::exercise_009(),
            "45" => PartialMove::exercise_001(),
            "46" => PartialMove::exercise_002(),
            "47" => PartialMove::exercise_003(),
            "48" => ReferenceBorrowing::exercise_001(),
            "49" => ReferenceBorrowing::exercise_002(),
            "50" => ReferenceBorrowing::exercise_003(),
            "51" => ReferenceBorrowing::exercise_004(),
            "52" => ReferenceBorrowing::exercise_005(),
            "53" => ReferenceBorrowing::exercise_006(),
            "54" => ReferenceBorrowing::exercise_007(),
            "55" => ReferenceBorrowing::exercise_008(),
            "56" => ReferenceBorrowing::exercise_009(),
            "57" => ReferenceBorrowing::exercise_010(),
            "58" => ReferenceBorrowing::exercise_011(),
            "59" => CompoundTypes::exercise_001(),
            "60" => CompoundTypes::exercise_002(),
            "61" => CompoundTypes::exercise_003(),
            "62" => CompoundTypes::exercise_004(),
            "63" => CompoundTypes::exercise_005(),
            "64" => CompoundTypes::exercise_006(),
            "65" => CompoundTypes::exercise_007(),
            "66" => CompoundTypes::exercise_008(),
            "67" => CompoundTypes::exercise_009(),
            "68" => CompoundTypes::exercise_010(),
            "69" => CompoundTypes::exercise_011(),
            "70" => CompoundTypes::exercise_012(),
            "71" => Arrays::exercise_001(),
            "72" => Arrays::exercise_002(),
            "73" => Arrays::exercise_003(),
            "74" => Arrays::exercise_004(),
            "75" => Arrays::exercise_005(),
            "76" => Arrays::exercise_006(),
            "77" => Slices::exercise_001(),
            "78" => Slices::exercise_002(),
            "79" => Slices::exercise_003(),
            "80" => Slices::exercise_004(),
            "81" => Slices::exercise_005(),
            "82" => Slices::exercise_006(),
            "83" => Tuples::exercise_001(),
            "84" => Tuples::exercise_002(),
            "85" => Tuples::exercise_003(),
            "86" => Tuples::exercise_004(),
            "87" => Tuples::exercise_005(),
            "88" => Tuples::exercise_006(),
            "89" => Structs::exercise_001(),
            "90" => Structs::exercise_002(),
            "91" => Structs::exercise_003(),
            "92" => Structs::exercise_004(),
            "93" => Structs::exercise_005(),
            "94" => Structs::exercise_006(),
            "95" => Structs::exercise_007(),
            "96" => Structs::exercise_008(),
            "97" => Enums::exercise_001(),
            "98" => Enums::exercise_002(),
            "99" => Enums::exercise_003(),
            "100" => Enums::exercise_004(),
            "101" => Enums::exercise_005(),
            "102" => Enums::exercise_006(),
            "103" => ControlFlow::exercise_001(),
            "104" => ControlFlow::exercise_002(),
            "105" => ControlFlow::exercise_003(),
            "106" => ControlFlow::exercise_004(),
            "107" => ControlFlow::exercise_005(),
            "108" => ControlFlow::exercise_006(),
            "109" => ControlFlow::exercise_007(),
            "110" => ControlFlow::exercise_008(),
            "111" => ControlFlow::exercise_009(),
            "112" => ControlFlow::exercise_010(),
            "113" => ControlFlow::exercise_011(),
            "114" => PatternMatch::exercise_001(),
            "115" => PatternMatch::exercise_002(),
            "116" => PatternMatch::exercise_003(),
            "117" => PatternMatch::exercise_004(),
            "118" => PatternMatch::exercise_005(),
            "119" => PatternMatch::exercise_006(),
            "120" => PatternMatch::exercise_007(),
            "121" => PatternMatch::exercise_008(),
            "122" => PatternMatch::exercise_009(),
            "123" => Patterns::exercise_001(),
            "124" => Patterns::exercise_002(),
            "125" => Patterns::exercise_003(),
            "126" => Patterns::exercise_004(),
            "127" => Patterns::exercise_005(),
            "128" => Patterns::exercise_006(),
            "129" => MethodsAndFuncions::exercise_001(),
            "130" => MethodsAndFuncions::exercise_002(),
            "131" => MethodsAndFuncions::exercise_004(),
            "132" => MethodsAndFuncions::exercise_005(),
            "133" => MethodsAndFuncions::exercise_006(),
            "134" => Generics::exercise_001(),
            "135" => Generics::exercise_002(),
            "136" => Generics::exercise_003(),
            "137" => Generics::exercise_004(),
            "138" => Generics::exercise_005(),
            "139" => Generics::exercise_006(),
            "140" => Generics::exercise_007(),
            "141" => ConstGenerics::exercise_001(),
            "142" => Traits::exercise_001(),
            "143" => Traits::exercise_002(),
            "144" => Traits::exercise_003(),
            "145" => Traits::exercise_004(),
            "146" => Traits::exercise_005(),
            "147" => Traits::exercise_006(),
            "148" => Traits::exercise_007(),
            "149" => Traits::exercise_008(),
            "150" => AssociatedTypes::exercise_001(),
            "151" => AssociatedTypes::exercise_002(),
            "152" => AssociatedTypes::exercise_003(),
            "153" => AssociatedTypes::exercise_004(),
            "154" => AssociatedTypes::exercise_005(),
            "155" => CollectionTypes::exercise_001(),
            "156" => CollectionTypes::exercise_002(),
            "157" => CollectionTypes::exercise_003(),
            "158" => CollectionTypes::exercise_004(),
            "159" => CollectionTypes::exercise_005(),
            "160" => CollectionTypes::exercise_006(),
            "161" => Vectors::exercise_001(),
            "162" => Vectors::exercise_002(),
            "163" => Vectors::exercise_003(),
            "164" => Vectors::exercise_004(),
            "165" => Vectors::exercise_005(),
            "166" => Vectors::exercise_006(),
            "167" => Vectors::exercise_007(),
            "168" => Vectors::exercise_008(),
            "169" => HashMaps::exercise_001(),
            "170" => HashMaps::exercise_002(),
            "171" => HashMaps::exercise_003(),
            "172" => HashMaps::exercise_004(),
            "173" => HashMaps::example_001(),
            "174" => HashMaps::exercise_005(),
            "175" => TypeCoercions::exercise_001(),
            "176" => TypeCoercions::exercise_002(),
            "177" => TypeCoercions::exercise_003(),
            "178" => TypeCoercions::exercise_004(),
            "179" => TypeCoercions::exercise_005(),
            "180" => FromIntoConversion::exercise_001(),
            "181" => FromIntoConversion::exercise_002(),
            "182" => FromIntoConversion::exercise_003(),
            "183" => FromIntoConversion::exercise_004(),
            "184" => FromIntoConversion::exercise_005(),
            "185" => Others::exercise_001(),
            "186" => Others::exercise_002(),
            "187" => Panics::exercise_001(),
            "188" => Panics::exercise_002(),
            "189" => Results::exercise_001(),
            "190" => Results::exercise_002(),
            "191" => Results::exercise_003(),
            "192" => Results::exercise_004(),
            "193" => Results::exercise_005(),
            "194" => Results::exercise_006(),
            "195" => FormatPrint::exercise_001(),
            "196" => FormatPrint::exercise_002(),
            "197" => DebugDisplay::exercise_001(),
            "198" => DebugDisplay::exercise_002(),
            "199" => DebugDisplay::exercise_003(),
            "200" => DebugDisplay::exercise_004(),
            "201" => DebugDisplay::exercise_005(),
            "202" => Lifetimes::exercise_001(),
            "203" => Lifetimes::example_001(),
            "204" => Lifetimes::exercise_002(),
            "205" => Lifetimes::example_002(),
            "206" => Lifetimes::exercise_003(),
            "207" => Lifetimes::exercise_004(),
            "208" => Lifetimes::exercise_005(),
            "209" => Lifetimes::exercise_006(),
            "210" => Lifetimes::exercise_007(),
            "211" => Lifetimes::exercise_008(),
            "212" => Lifetimes::example_003(),
            "213" => Lifetimes::exercise_009(),
            "214" => Lifetimes::exercise_010(),
            "215" => StaticLifetimes::exercise_001(),
            "216" => StaticLifetimes::exercise_002(),
            "217" => StaticLifetimes::exercise_003(),
            "218" => StaticLifetimes::example_001(),
            "219" => StaticLifetimes::exercise_005(),
            "220" => StaticLifetimes::exercise_006(),
            "221" => Closures::exercise_001(),
            "222" => Closures::exercise_002(),
            "223" => Closures::exercise_003(),
            "224" => Closures::exercise_004(),
            "225" => Closures::exercise_005(),
            "226" => Closures::exercise_006(),
            "227" => Closures::exercise_007(),
            "228" => Closures::exercise_008(),
            "229" => Closures::exercise_009(),
            "230" => Closures::exercise_010(),
            "231" => Closures::exercise_011(),
            "232" => Iterators::exercise_001(),
            "233" => Iterators::exercise_002(),
            "234" => Iterators::exercise_003(),
            "235" => Iterators::exercise_004(),
            "236" => Iterators::exercise_005(),
            "237" => Iterators::exercise_006(),
            "238" => Iterators::example_001(),
            "239" => Iterators::exercise_007(),
            "240" => Iterators::exercise_008(),
            "241" => Iterators::exercise_009(),
            "242" => Iterators::exercise_010(),
            _ => {}
        }
        if interact.is_empty | interact.is_numeric {
            println!("");
            break
        }
    }
}