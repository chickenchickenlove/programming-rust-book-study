use std::cell::RefCell;

struct Cache {
    store : RefCell<Option<i32>>
}

impl Cache {

    fn new() -> Cache {
        Cache { store: RefCell::new(None) }
    }

    fn get_or_default(&self, compute: impl FnOnce() -> i32) -> i32 {
        if self.store.borrow().is_none() {
            let result = compute();
            *self.store.borrow_mut() = Some(result);
        }
        self.store.borrow().unwrap()
    }
}


fn main() {
    let cache = Cache::new();

    let result = cache.get_or_default(|| {
        println!("Computing...");
        42
    });
    println!("{}", result);

    let mut box1 = Box::new(2);

    println!("{}", box1);
    *box1.as_mut() = 10;

    println!("{}", box1);
}
