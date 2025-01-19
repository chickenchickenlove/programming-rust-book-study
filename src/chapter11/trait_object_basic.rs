use std::io::Write;

pub fn hello() {
    let mut buf: Vec<u8> = vec![];

    // 아래 코드에서 컴파일 에러 발생.
    // dyn Write는 컴파일 시점에 크기를 알 수 없다.
    // 구현체는 여러 필드를 가지고 있어서, 스택 프레임에 배정된 크기가 다를 수 있기 때문이다.
    // let writer: dyn Write = buf;

    // Reference는 고정된 크기를 가지므로 문제가 없다.
    let writer: &mut dyn Write = &mut buf;
}


// 함수가 Trait Object를 원할 때, 참조자를 넘기면 암묵적으로 Trait Object로 변환됨.
fn need_trait_object(writer: &mut dyn Write) { }
fn other_function() {
    let mut buf: Vec<u8> = vec![];
    need_trait_object(&mut buf);
}


