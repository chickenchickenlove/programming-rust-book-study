struct T1{}

trait Hello {
    fn hello(&self) -> String; }

impl Hello for T1 {
    fn hello(&self) -> String { todo!() } }

fn do_it(trait_object: &dyn Hello) { trait_object.hello(); }

fn main() {
    let k = T1 {};
    // 컴파일 타임 -> 여기서 &k가 &dyn Hello로 Trait Object 변환되는 것을 확인함.
    // 컴파일러는 이 때, VTable for (Hello, T1)를 생성한다.
    // 컴파일 타임에 이미 &k가 &dyn Hello로 전환되는 시점에 어떤 VTable이 포함되어야 할지 안다.
    // 컴파일 시점에 필요한 VTable이 생성되고, 어떤 VTable이 사용될지 결정된다.
    // 런타임에는 Trait Object 변환 시, 단순히 VTable의 주소만 붙인 Fat Pointer가 생성되는 정도다.
    do_it(&k)
}
