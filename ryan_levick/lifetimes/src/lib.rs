use core::slice;

struct MyIterator<'a, T> {
    // 1)
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    // 2)
    type Item = &'a T;
    // 4)
    fn next(&mut self) -> Option<Self::Item> {
        // 3)
        if self.slice.is_empty() {
            return None;
        }
        // get the first element.
        let element = self.slice.get(0);
        // set self.slice = to the other elements.
        self.slice = &self.slice[1..];
        // return the first element.
        element
    }
}

struct MyMutableIterator<'iter, T> {
    // 5)
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        // 6)
        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut collection = vec![1,2,3,4];
        let wrapper = MyIterator {
            slice: &collection[..],
        };
        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        let mut collection = vec![1,2,3,4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };
        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }
        assert_eq!(collection.get(0), Some(&2));
    }
}

/* 
6)
    Does not compile because the lifetime of self is tied to next and does not last as long as 
    MyMutableIterator.
    get first element.
    let element = self.slice.get_mut(0);
    set self.slice to the rest of the list.
    self.slice = &mut self.slice[1..];
    return first element.
    element
    Gets around this by replacing self.slice with a dummy variable then after working with it passes
    the updated value back to self.slice.
    let slice = slice is called shadowing variables (overwriting them).
    std::mem::replace replaces the pointer slice to a pointer of an empty list then returns slice that lives 
    for 'iter lifetime.
5)
    slice is exclusive. Can only have one mutable/exlucise reference.
4)
    Iterators are meant to be used only once (start to finish). It is not changing the slice itself, 
    but changing the pointer.
3) 
    Another Way:
    Returns early if None otherwise returns an option
    let (element, rest) = self.slice.split_first()?;
    self.slice = rest;
    Some(element);
2)
    The element will live for as long as the wrapper does.
1)
    When you implement a field that is borrowed, you have to specify the 
    lifetime of that field. 
    Using <'a> has tied the lifetime of MyIterWrapper to 
    the slice within it.
    slice is non exclusive. Can have many non mutable/non-exclussive references.
*/