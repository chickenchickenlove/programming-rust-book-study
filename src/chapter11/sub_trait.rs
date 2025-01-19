trait Visible {
    fn hello(&self);
    fn hello_default() {
        println!("hello_default");
    }

}

trait Creature: Visible {
    fn hello_creature(&self);
}

struct Struct1 {}

impl Creature for Struct1 {
    fn hello_creature(&self) {
        todo!()
    }
}

// Creature를 구현하려면 반드시 Visible도 함께 구현해야 문제가 없다.
impl Visible for Struct1 {
    fn hello(&self) {
        todo!()
    }
}
