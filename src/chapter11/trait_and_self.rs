struct S1 {}
struct S2 {}


pub trait Concat {
    fn concat(&self, other: &Self) -> &Self;
}

// Trait Object S1, S2를 위한 VTable을 생성할 때, concat() 메서드의 시그니처가 모두 동일해야한다.
// 그러나 S1은 fn concat(&self, other: &S1) -> &S1
// S2는 fn concat(&self, other: &S2) -> &S2
// 가 되어서 메서드 시그니처가 달라진다. 따라서 VTable을 생성할 수 없게 된다.
impl Concat for S1 {
    fn concat(&self, other: &Self) -> &Self {
        self
    }
}

impl Concat for S2 {
    fn concat(&self, other: &Self) -> &Self {
        self
    }
}

fn concat_do_do(s1: &dyn Concat, s2: &dyn Concat) {
    let k = s1.concat(s2);
}

pub fn do_it() {
    let mut s1 = S1 {};
    let mut s2 = S2 {};
    concat_do_do(&s1, &s2);
}




