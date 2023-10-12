// 3)
pub fn spellcheck<C: Spellchecker>(input: &str, spellchecker: C) -> String {
    let mut result = input.to_owned();
    // 1)
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

// 2)
pub fn spellcheck2(input: &str, spellchecker: &dyn Spellchecker) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

struct NoopSpellchecker;

impl Spellchecker for NoopSpellchecker {
    // 6)
    fn check(&self, input: &str) -> Vec<Change> {
        Vec::new()
    }
}

struct AntispaceChecker;

impl Spellchecker for AntispaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| Change::Delete(index..index + space.len()))
            .collect()
    }
}

// Takes a achange and updates the string with that change.
fn apply_change(string: &mut String, change: Change) {
    // TODO: implement
}

pub enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

struct TextEditor {
    buffer: String,
    pub spellchecker: Box<dyn Spellchecker>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello, is it me you're looking for?";
        let result = spellcheck(text, NoopSpellchecker);
        assert!(result == text);
        let result = spellcheck(text, AntispaceChecker);
        assert!(result == text);
        spellcheck2(text, &NoopSpellchecker);
        spellcheck2(text, &AntispaceChecker);
        // 4)
        let spellcheckers: Vec<Box<dyn Spellchecker>> = vec![
            Box::new(NoopSpellchecker), Box::new(AntispaceChecker)
        ];
        for sp in spellcheckers {
            // 5)
            spellcheck2(text, &*sp);
        }
    }
}


/* 
6)
    The annotation #[inline(always)] tells the compiler do this optimisation no matter what even when
    in debug mode. Don't compile down to a function call and where ever the following method is being
    used, just copy paste it in. Only works in static dispatch.
5)
    * de-refernces the object and then & re-references it and here it de-references the box and 
    re-refernces it borrowing the contents of the Box<>. Or use .as_ref().
4)
    Cannot construct a vector with 2 different types in it. As they might have diferent sizes in 
    memory. Specifying Vec<Box<dyn Spellchecker>> tells the compiler that the vector contains the 
    same types. Now each element is an owned pointer.
3)
    Everytime the function is used with a diferent type it gets copy pasted into the binary and the 
    binary grows larger when compiled. It is a static dispatch and has less runtime cost. Rust can 
    easily look through the method and see if it can just paste in the code in place of it's 
    reference. Static dispatch will be slower to compile.
2)
    Uses dynamic dispatch instead of static dispatch. dyn is explicit when using a trait object. In 
    Java almost everything is heap allocated except primitive types, but in rust almost everything unless specified is stack 
    allocated. Rust needs to know how much space to allocate for functions and their parmeters. A trait 
    is just a contract and can't be passed and when running only pass the types that implement that 
    trait.
    A struct only takes up space on the stack when it contains fields and the field types determine 
    the amount of space taken up.
    Generic objects copy pastes it's parameter types on to the stack and the size of those are already 
    known. A trait object needs the space to be specified for it's parameter types that are being 
    referenced. Using Box<> allocates the interior onto the heap and is a pointer. A pointer always has 
    the same size. Box<> owns the reference/pointer and & borrows.
    Has more runtime cost as it has to go through a V-Table to find where the types live in memory.
    Dynamic dispatch can also unlock additional functionality that is not available with static 
    dispatch.
    Dynamic dispatch does not allow for inlining.
1)
    Relies on static dispatch. For each time spellcheck is called with a diferent spellchecker the 
    rust compiler will create specialised spellcheck functions for each one of those spellcheckers 
    called monomorphisation copy paste the implementation and differs them based on the two types 
    being called with.
*/