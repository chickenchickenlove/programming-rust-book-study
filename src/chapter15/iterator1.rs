fn hello() {

    let values = vec!["a".to_string(), "b".to_string()];

    // * Trait
    // IntoIterator :
    //   - 값을 하나 받아서, 그 값을 이용해 Iterator를 반환하는 기능을 제공. into_iter()를 통해서 반환함.
    //   - for element in &collection { ... } -> Shared Reference Item -> Iterator
    //   - for element in &mut collection { ... } -> Mutable Reference Item
    //   - for element in collection { ... } -> Value Item
    //   - 특정 Iterator는 특별한 이유로 일부 형태는 지원하지 않는다. (예를 들어 HashSet은 요소가 변경될 경우, Hash 값이 바뀔 수 있기 때문에 Mutable Reference Iterator는 지원하지 않음)
    // Iterator :
    //   - 주로 next() 메서드를 호출해서 다음 Item을 반환하는 Trait.
    //   - 대부분의 Iterator 구현체는 IntoIterator도 구현함. 이 경우, 자기 자신을 반환하도록 구현되어 있음.
    // 동일한 형태
    // - (&a).into_iter(), a.iter(), for b in &a;
    // - (&mut a).into_iter(), a.iter_mut(), for b in &mut a;
    // - (a).into_iter(), for b in a;


    // for 문은 항상 전달된 값의 into_iter() 메서드를 호출한다.
    // into_iter() 메서드의 좌항이 어떤 타입이냐에 따라 into_iter()는 서로 다른 이터레이터를 반환한다.
    // 1. Shared Reference -> Shared Reference Item을 반환하는 Iterator
    // 2. Mutable Reference -> Mutable Reference Item을 반환하는 Iterator
    // 3. Value -> Value Item을 반환하는 Iterator

    // 동치 표현 - Shared Reference
    let shared_reference_iterator  = values.iter();
    let shared_reference_iterator1  = (&values).into_iter();
    for value in shared_reference_iterator1 { }
    for value in &values { }
    for value in values.iter() { }

    // 동치 표현 - Mutable Reference
    let mutable_reference_iterator  = values.iter_mut();
    let mutable_reference_iterator1 = (&mut values).into_iter();
    for value in mutable_reference_iterator1 { }
    for value in &mut values { }
    for value in values.iter_mut() { }

    // 동치 표현 - Moved
    let moved_iterator = (values).into_iter();
    for value in moved_iterator { }
    for value in values { }
}



