struct Struct1 {}
struct Struct2 {}
trait MyInterface {
    fn hello() { println!("Parent Hello"); }
}

impl MyInterface for Struct1 {}
impl MyInterface for Struct2 {
    fn hello() {
        println!("It's implemented");
    }
}

pub fn default_call() {
    Struct1::hello();
    Struct2::hello();
}