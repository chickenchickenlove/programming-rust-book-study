
use std::ops::Add;

// trait Shape { }
//
// struct Circle {}
// struct Triangle {}
// struct Rectangular {}

// impl Shape for Circle {
//     fn new() -> Self {
//         Circle {}
//     }
// }
//
// impl Shape for Triangle {
//     fn new() -> Self {
//         Triangle {}
//     }
// }
//
// impl Shape for Rectangular {
//     fn new() -> Self {
//         Rectangular {}
//     }
// }

// pub fn make_shape(shape: &str) -> impl Shape {
//     match shape {
//         "circle" => Circle::new(),
//         "triangle" => Triangle::new(),
//         _ => Rectangular::new(),
//     }
// }

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {}
struct Triangle {}
struct Rectangular {}

impl Circle {
    fn new() -> Circle {
        Circle {}
    }
}

impl Triangle {
    fn new() -> Triangle {
        Triangle {}
    }
}

impl Rectangular {
    fn new() -> Rectangular {
        Rectangular {}
    }
}


impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Shape for Rectangular {
    fn area(&self) -> f64 {
        todo!()
    }
}

// 반환 타입에 impl Shape를 사용할 수는 있지만, impl Shape는 정적 디스패치다.
// 따라서 반환 타입은 반드시 하나가 될텐데, 이 경우 impl Shape를 사용하거나, 실제 타입을 반환해주는 것은 큰 차이가 없다.
pub fn make_shape2(shape: &str) -> impl Shape {
    match shape {
        "circle" => Circle::new(),
        "triangle" => Circle::new(),
        _ => Circle::new(),
    }
}

// 만약 Shape Trait을 구현한 서로 다른 타입을 반환한다면, 동적 디스패치 타입을 반환하도록 한다.
pub fn make_shape(shape: &str) -> Box<dyn Shape> {
    match shape {
        "circle" => Box::new(Circle::new()),
        "triangle" => Box::new(Triangle::new()),
        _ => Box::new(Rectangular::new()),
    }
}


// pub fn make_shape(shape: &str) -> impl Shape {
//     match shape {
//         "circle" => Box::new(Circle::new()),
//         "triangle" => Box::new(Triangle::new()),
//         _ => Box::new(Rectangular::new()),
//     }
// }


use std::iter;
use std::vec::IntoIter;

// 이 코드는 반환 타입에 대한 가독성이 나쁘다.
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) ->  iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 가독성을 높이기 위해 Trait Object로 바꿔서 반환할 수 있음.
// 그런데 고작 이거 개선하려고 동적 디스패치로 성능을 낮게 가져가는 것은 말도 안됨.
fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) ->  Box<dyn Iterator<Item=u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

// 가독성을 높이기 위해 impl Trait을 사용함. (좀 더 추상화 됨)
// 이 함수를 사용하는 쪽에서는 반환되는 값을 Iterator<u8>을 구현하는 타입이라고 가정하고 사용해도
// 문제없이 컴파일 된다.
fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) ->  impl Iterator<Item=u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}