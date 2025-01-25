use std::fmt::{Debug, Display};

trait MyIterator {
    // 연관타입 Item
    type Item;

    // 연관타입을 반환.
    // Self::Item은 독립적인 타입이 아니라, 각 Iterator 구현에 종속적인 것을 의미함.
    fn next(&mut self) -> Option<Self::Item>;
}

struct Args{}

impl MyIterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

// 제네릭 코드에서도 연관 타입을 사용할 수도 있음.
// 이 함수는 문맥상 Iterator가 가진 요소들을 반환하는 것이기 때문에 iterator의 연관타입 item을 사용함.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

// fn dump<I>(iter: I)
//     where I: Iterator
// {
//     // 아래 코드는 컴파일 되지 않는다.
//     // Iterator가 제공하는 value의 타입이 Debug Trait을 구현하는지 알 수 없음.
//     for (index, value) in iter.enumerate() {
//         println!("{}, {:?}", index, value);
//     }
// }

fn dump1<I>(iter: I)
    where I: Iterator, I::Item: Debug // I가 반환하는 Item 타입이 Display 가능한지를 표현해준다.
{
    for (index, value) in iter.enumerate() {
        println!("{}, {:?}", index, value);
    }
}

// 혹은 연관타입 Item을 다음과 같이 처리할 수도 있음.
fn dump2<I>(iter: I)
    where I: Iterator<Item=String>
{
    for (index, value) in iter.enumerate() {
        println!("{}, {:?}", index, value);
    }
}


pub fn do_my_iterator() {
    let k1 = vec!["a", "b", "c"];
    dump1(k1.iter());

    let k2 = vec!["d".to_string()];
    dump2(k2.into_iter());
}

// RHS = Right hand side
pub trait MyMul<RHS=Self> {
    // 연산 이후의 결과 타입.
    type Output;
    // * 연산자를 위한 메서드.
    fn mul(self, rhs: RHS) -> Self::Output;
}

struct MyComplex {}

// RHS=Self로 구현되어있기 때문에
// impl MyMul for MyComplex는 impl MyMul<Complex> for MyComplex와 동일하다.
// 여기서 RHS=Self는 fn mul()의 rhs의 타입에 사용된다.
impl MyMul for MyComplex {
    type Output = MyComplex;
    fn mul(self, rhs: MyComplex) -> Self::Output {
        todo!()
    }
}
