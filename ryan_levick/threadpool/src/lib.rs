use std::sync::{mpsc::{channel, Sender}, Mutex, Arc, atomic::{AtomicU32, Ordering}};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];
        for _ in 0..num_threads {
            let clone = receiver.clone();
            // 1)
            let handle = std::thread::spawn(move || loop {
                // check for work
                let work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => break
                };
                println!("Start");
                // one work, do work
                work();
                println!("Finish");
            });
            _handles.push(handle);
        }
        Self {
            _handles,
            sender,
        }
    }
    // 2)
    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}


pub struct ThreadPool1 {
    _handles: Vec<std::thread::JoinHandle<()>>,
    // 4)
    sender: Sender<Box<dyn FnMut() + Send>>
}

impl ThreadPool1 {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn FnMut() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];
        for _ in 0..num_threads {
            let clone = receiver.clone();
            // 1)
            let handle = std::thread::spawn(move || loop {
                // check for work
                let mut work = match clone.lock().unwrap().recv() {
                    Ok(work) => work,
                    Err(_) => break
                };
                println!("Start");
                // one work, do work
                work();
                println!("Finish");
            });
            _handles.push(handle);
        }
        Self {
            _handles,
            sender,
        }
    }
    // 2)
    pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        let foo = || println!("Hello from thread");
        pool.execute(foo);
        pool.execute(|| println!("Hello from thread"));
        let foo = || std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
        pool.execute(foo);
        let mut n = AtomicU32::new(0);
        let nref = Arc::new(n);
        let pool = ThreadPool1::new(10);
        let clone = nref.clone();
        // 3)
        let foo = move || {
            clone.fetch_add(1, Ordering::SeqCst);
        };
        pool.execute(foo.clone());
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(nref.load(Ordering::SeqCst), 2);
        let foo = || println!("Hello from thread");
        pool.execute(foo);
        pool.execute(|| println!("Hello from thread"));
    }
}

/* 
4)
    FnMut is a sub function of Fn and gives us more functionality.
3)
    Arc allows multiple owners but does not give exclusive access. So, the object inside Arc cannot be 
    mutated. For numbers Atomic allows modification of objects without exclusive access. Atomic like 
    Mutexes but CPU guarantees access.
2)
    'static enables the generic object to live for as long as needed.
1)
    Tells compiler not to reference an object using the minimum amount needed to get things to want, 
    but to take ownership.
*/