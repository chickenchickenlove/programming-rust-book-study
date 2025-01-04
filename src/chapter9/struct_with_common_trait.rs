
// #derive를 이용하면 Common Trait을 자동으로 구현해준다.
// 1. Struct가 Common Trait을 구현하기 위해서는 각 필드가 그 Trait을 구현하고 있어야 한다.
// 2. 번거롭게 명시하도록 하는 것은, derive()를 사용하는 것만으로 관련된 Commono Trait이 Public API가 되기 때문이다. 이것은 이후에 변화에 취약해짐을 의미한다.
// Common Trait을 Copy, Clone, Debug, PartialEq 등이 존재한다.
#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn equal() {
    let a = Point{ x: 123, y: 456 };
    let b = Point{ x: 123, y: 456 };
    let eq = a==b;
}

