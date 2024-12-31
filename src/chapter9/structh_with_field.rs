
pub struct GrayscaleMap {
    // 필드를 private으로 하는 경우.
    pixels: Vec<u8>,
    size: (usize, usize)

    // 필드를 public으로로 하는 경우
    // pub pixels: Vec<u8>,
    // pub size: (usize, usize),
}

impl GrayscaleMap {
    // 필드가 private인 경우에는 Struct를 생성조차 할 수 없다.
    // 따라서 관습적으로 New() 메서드를 제공한다.
    fn new(pixels: Vec<u8>, size: (usize, usize)) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { size, pixels }
    }
}


pub fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap{
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap{ pixels, size }
}


struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height/2, ..b };
    let mut broom2 = Broom { name: broom1.name.clone(), ..broom1 };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

pub fn broom_test() {
    let broom = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (b1, b2) = chop(broom);
    assert_eq!(b1.name, "Hokey I");
    assert_eq!(b1.height, 30);
    assert_eq!(b1.health, 100);

    assert_eq!(b2.name, "Hokey II");
    assert_eq!(b2.height, 30);
    assert_eq!(b2.health, 100);
}

