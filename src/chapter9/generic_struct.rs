
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {

    // Queue<T> 대신 Self 타입을 사용할 수 있음.
    pub fn new() -> Self {
        // Queue 대신 Self를 사용할 수 있음.
        Self { older: vec![], younger: vec![]}
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

fn he() {
    let mut q =Queue::new();
    q.push('a');
    q.push('b');

    assert_eq!(q.pop(), Some('a'));
    assert_eq!(q.pop(), Some('b'));

    let (older, younger) = q.split();
}



