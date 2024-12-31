
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {

    pub fn new() -> Queue {
        Queue { older: vec![], younger: vec![]}
    }

    // 메서드가 자기 자신의 소유권을 가져야 하는 경우
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
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
    let mut q = Queue::new();
    // push()는 &mut self를 원한다. self.push()를 이용하더라도, 자동으로 수정해서 전달한다.
    // 여기서 사용자가 굳이 (&mut q).push()를 호출하지 않아도 된다.
    q.push('a');
    q.push('b');

    assert_eq!(q.pop(), Some('a'));
    assert_eq!(q.pop(), Some('b'));

    // 여기서 q는 미초기화 상태가 됨. split()이 self를 받기 때문임.
    let (older, younger) = q.split();
}


