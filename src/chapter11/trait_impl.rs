use std::fmt::Write;
use crate::chapter11::trait_object_basic::hello;

struct Canvas{}

trait Visible {
    fn draw(&self, canavas: &mut Canvas);
}

struct Broom{}

impl Visible for Broom {
    // Impl ... For ... 에서는 Trait 메서드만 구현 가능.
    // 다른 메서드는 다른 Impl Block에서 생성.
    // Impl ... For ... 에서 Impl Block에 있는 메서드 호출 가능함.
    fn draw(&self, canavas: &mut Canvas) { hello() }
}

impl Broom {
    fn hello() { }
}

// Write Trait을 구현하는 타입들을 위해 Visible Trait을 구현할 수도 있음.
impl<W: Write> Visible for W {
    fn draw(&self, canavas: &mut Canvas) {
        todo!()
    }
}

