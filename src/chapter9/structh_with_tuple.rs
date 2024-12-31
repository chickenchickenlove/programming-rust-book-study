


struct Bounds(usize, usize);

// 필드 일부만 public
// struct Bounds(pub usize, usize);

// 필드 전체 public
// struct Bounds(pub usize, pub usize);


fn t() {
    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0, 1024);
    assert_eq!(image_bounds.1, 768);
}

// 튜플형 스트럭트는 타입 검사를 위해 주로 선언되는 NewType에 주로 사용할 수 있음.
// struct Ascii는 일종의 NewType이다.
struct Ascii(Vec<u8>);

fn t1() {
    let v1: Vec<u8> = vec![0, 1, 2, 3];
    // 이 코드만으로는 Vec<u8> 타입이 뭘 의미하는지 알 수 없다.
    let vec1: Vec<u8> = vec![0, 1, 2, 3];

    let v2: Vec<u8> = vec![0, 1, 2, 3];
    // Ascii 타입이란 것을 알려주면서, 이 값이 무엇을 의미하는지 더 명확해졌다.
    let ascii = Ascii(v2);
}


