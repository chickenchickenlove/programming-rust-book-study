
trait StringSet {
    // 다음 코드는 Trait Object를 사용할 때는 VTable에 포함될 수 없음.
    // 이 함수를 구현하는 구현체마다, Self의 타입이 달라지기 때문에 new()의 메서드 시그니처가 달라져 VTable에 포함될 수 없음.
    // fn new() -> Self;
    // Self where Self: Sized;로 작성해주면 &dyn MyTrait 같은 Trait Object는 Sized와는 관련이 없으므로,
    // &dyn MyTrait에 대한 VTable에 new() 함수를 넣지 않으면서 컴파일 에러가 해결된다.
    fn new() -> Self where Self: Sized;

    fn contains(&self, string: &str) -> bool;
}

fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new(); // 타입 연관 함수도 이렇게 호출할 수 있음.
    for word in document {
        if !wordlist.contains(word) { // continue }
        }
    };
}



