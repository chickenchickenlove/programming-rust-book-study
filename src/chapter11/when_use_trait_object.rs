
struct Carrot {}
struct Apple {}

trait Vegetable {}
impl Vegetable for Carrot {}
impl Vegetable for Apple {}

// 이렇게 작성된 경우, Salad의 Veggis는 한 가지 타입만 가능함.
struct Salad<V: Vegetable> {
    veggies: Vec<V>
}

fn only_one_salad() {
    let apples: Vec<Apple> = vec![];
    let salad = Salad { veggies: apples };
}


// 이건 불가능함, Trait Object는 타입 매개변수로 들어갈 수 없음.
// struct SaladDynamic<V: &dyn Vegetable> { veggies: Vec<V> }

// 이것도 불가능함. dyn Vegetable은 struct 그 자체를 의미하기 때문에 크기가 다를 수 있음.
// struct SaladDynamic { veggies: Vec<dyn Vegetable> }

// Trait Object를 전달하면 사용할 수 있음.
struct SaladDynamic {
    veggies: Vec<Box<dyn Vegetable>>
}
fn only_multiple_salad() {
    let mut veggies: Vec<Box<dyn Vegetable>> = vec![];
    veggies.push(Box::new(Apple {}));
    veggies.push(Box::new(Carrot {}));

    let salad = SaladDynamic { veggies: veggies };
}



