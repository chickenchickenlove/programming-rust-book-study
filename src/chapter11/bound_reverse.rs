use std::ops::{Add, Mul};

// 이 함수를 제네릭으로 만들고 싶다.
fn dot1(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0 .. v1.len() {
        total = total + v1[i] + v2[i];
    }
    total
}

// 제네릭 N을 넣었지만, 제네릭 N이 Copy 가능한지 알 수 없다.
// 따라서 v1[i]라고 하면 Move를 하려고 해서 컴파일러가 거절한다.
// Copy Trait을 구현하게 되면, Move 대신에 값을 복사해서 전달한다.
fn dot2<N>(v1: &[N], v2: &[N]) -> N
    where N: Copy

{
    let mut total: N = 0;
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}


// 제네릭 N이 곱하거나 더할 수 있는 값이라는 것을 알 수 없다.
fn dot3<N>(v1: &[N], v2: &[N]) -> N
    where N: Mul<Output=N> + Add<Output=N> + Copy
{
    let mut total: N = 0;
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}


// N 타입의 total이라는 변수가 0을 가질 수 있는지 알 수 없다.
// 따라서 적절한 Default Type을 넣어주는게 좋다.
fn dot4<N>(v1: &[N], v2: &[N]) -> N
    where N: Mul<Output=N> + Add<Output=N> + Copy + Default
{
    let mut total: N = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}
