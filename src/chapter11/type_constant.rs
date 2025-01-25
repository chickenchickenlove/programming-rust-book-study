use std::ops::Add;

// Trait 같은 인터페이스 내부에 연관 상수를 선언할 수 있음.
trait MyFloat {
    // 연관 상수
    const ZERO: Self;
    const ONE: Self;
    const TWO: &'static i32 = &1;
}

// 연관상수도 상속받아서 오버라이드 할 수 있음.
impl MyFloat for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

// 연관 상수를 일반적인 메서드에서 사용할 수도 있음.
// 연관 상수를 사용할 때는 타입 정보가 필요하기 때문에, 컴파일 시점에 타입 정보를 알 수 있어야 한다.
// 따라서 연관 상수는 Trait Object에서는 사용할 수 없다.
fn fib<T: MyFloat + Add<Output=T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n-1) + fib::<T>(n-2)
    }
}


