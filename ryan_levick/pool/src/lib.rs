// 1)
use std::cell::RefCell;
struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T: PoolItem> Pool<T> {
    fn new() -> Self {
        Self { 
            items: RefCell::new(Vec::new()), 
        }
    }
    fn get(&self) -> PoolGuard<T> {
        // 2)
        let item = match self.items.borrow_mut().pop() {
            Some(item) => item,
            None => T::new(),
        };
        PoolGuard { 
            inner: Some(item),
            items: &self.items, 
        }
    }
}

trait PoolItem {
    fn new() -> Self;
    fn reset(&mut self);
}

struct PoolGuard<'a, T: PoolItem> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>,
}

impl<T: PoolItem> Drop for PoolGuard<'_, T> {
    // 3)
    fn drop(&mut self) {
        let mut item = self.inner.take().unwrap();
        item.reset();
        // somehoe return the item to the pool
        self.items.borrow_mut().push(item);
    }
}

impl<T: PoolItem> std::ops::Deref for PoolGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // 4)
        self.inner.as_ref().unwrap()
    }
}

impl<T: PoolItem> std::ops::DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct Awesome(usize);
        impl Awesome {
            fn get(&self) -> usize {
                self.0
            }
            fn inc(&mut self) {
                self.0 += 1;
            }
        }
        impl PoolItem for Awesome {
            fn new() -> Awesome {
                Awesome(0)
            }
            fn reset(&mut self) {
                self.0 = 0
            }
        }
        let pool: Pool<Awesome> = Pool::new();
        let mut item = pool.get();
        item.inc();
        assert_eq!(item.get(), 1);
        drop(item);
        let new_item = pool.get();
        assert_eq!(new_item.get(), 0);
    }
}

/*
4)
    Without .as_ref() attempting to move the inner field out from the struct, whereas it is being
    borrowed.
3)
    Will only be called once, therefore .unwrap() should not cause panic!().
2)
    .borrow_mut sets the state to being exclusively borrowed. Cannot have immutable and mutable 
    borrows at the same time, but can have many immutable borrows.
1)
    Allows access at runtime rather than compile time. It is not thread safe and not sync or send. The 
    struct it is in becomes not sync or send. Will only run in one thread.
*/