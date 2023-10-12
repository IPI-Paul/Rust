pub trait MyIterator: Sized {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    fn map<F, B>(self, f: F) -> MyMap<Self, F>
    where F: FnMut(Self::Item) -> B {
        MyMap { iterator: self, func: f }
    }
    fn collect<B>(self) -> B 
    where B: MyFromIterator<Self::Item>, {
        B::from_iter(self)
    }
}

impl MyFromIterator<String> for Vec<String> {
    fn from_iter<T>(mut my_iter: T) -> Vec<String>
    where T: MyIterator<Item = String> {
        let mut result = Vec::new();
        while let Some(item) = my_iter.next() {
            result.push(item)
        }
        result
    }
}

pub struct MyMap<I, F> {
    iterator: I,
    func: F,
}

impl<I: MyIterator, F: FnMut(I::Item) -> B, B> MyIterator for MyMap<I, F> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next()?;
        Some((self.func)(item))
    }
}
pub trait MyFromIterator<A> {
    fn from_iter<T>(my_iter: T) -> Self
    where T: MyIterator<Item = A>;
}

struct MyVecIterator<'a, T> {
    vec: &'a Vec<T>,
    current_index: usize,
}

impl<'a, T> MyIterator for MyVecIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // 1)
        // what current item?
        let current = self.current_index;
        // get the current item.
        // 4)
        let current_item = self.vec.get(current);
        // return it back.
        // 3)
        // 6)
        self.current_index += 1;
        current_item
    }
}

trait MyIteratorExt<T> {
    fn my_iter<'a>(&'a self) ->MyVecIterator<'a, T>;
}

impl<T> MyIteratorExt<T> for Vec<T> {
    fn my_iter<'a>(&'a self) -> MyVecIterator<'a, T> {
        MyVecIterator {
            vec: &self,
            current_index: 0,
        }
    }
}

// 8)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_vec = vec![1,2,3];
        // 2)
        println!("Iterate over vector using for loop.");
        for item in my_vec{
            println!("{}", item)
        }
        let vec = vec![1,2,3];
        let mut iter = MyVecIterator {
            vec: &vec,
            current_index: 0,
        };
        println!("Type MyVecIterator maintains state for vector and iterates using next implementation.");
        // 5)
        while let Some(item) = iter.next() {
            println!("{}", item);
        }
        // 7)
        let mut iter = vec.my_iter();
        println!("Type MyVecIterator maintains state for vector and iterates using trait extension and next implementation.");
        while let Some(item) = iter.next() {
            println!("{}", item);
        }
        let vec = vec![1usize,2,3];
        let mut iter = vec.my_iter().map(|x| x * 2);
        while let Some(item) = iter.next() {
            println!("{}", item);
        }
        // 9)
        let vec = vec![1usize,2,3];
        let mut new_vec: Vec<String> = vec.my_iter()
            .map(|x| x.to_string())
            .collect();
        println!("{:?}", new_vec);
    }
}

/*
9)
    Iterators are lazy, they don't do anything until actually called either in a loop or adding 
    .collect() that gets back a vector from them.
8)
    Trying to add the .into_iter functionality for for loops but could not get it to work.
    impl<T: MyIterator> Iterator for T {
        type Item = <T as MyIterator>::Item;
        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    impl<'a, T> IntoIterator for MyVecIterator<'a, T> {
        type Item = T;
        type IntoIter = MyVecIterator<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            todo!()
        }
    }
7)
    Cannot have while let Some(item) = vec.my_iter().next() {} as it just 
    ceates a new iterator every time through the loop.
6)
    Prevents infinite loop.
5)
    Causes an infinite loop.
4)
    .get() returns a pointer to an item. Does bounds checking.
3)
    unimplemented!() is like todo!() and just tells the compiler not to 
    highlight code errors in it's closure.
2)
    We can iterate through vector, but we are not calling next on vector. The for loop utilises 
    .into_iter on the vector.
1)
    Where do we keep the state of what item is being iterated on. Vectors 
    are not iterators and do not have an iterator trait.
    Need to create a new type that we can implement an iterator for that 
    keeps state as cannot keep state in vector.
*/