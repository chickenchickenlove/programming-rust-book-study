use std::io::Write;
use std::fmt::Debug;



// 제네릭을 이용한 다형성 -> 정적 디스패치 (Monomorphization)
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello World\n");
    out.flush()
}

// Trait Object을 이용한 다형성 -> 동적 디스패치
fn say_hello_with_trait_object(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello World\n");
    out.flush()
}


// 상위 10개의 값을 출력하고 싶다.
pub fn top_ten<T: Debug + Clone + Ord>(values: &Vec<T>) {
    let mut k = values.to_vec();
    k.sort();
    println!("{:?}", k);
}

// 가독성을 살리기 위해 Where절로 뺄 수도 있음.
pub fn top_ten_with_where<T>(values: &Vec<T>)
    where T: Debug + Clone + Ord
{
    let mut k = values.to_vec();
    k.sort();
    println!("{:?}", k);
}


