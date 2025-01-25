use std::os::unix::fs::chown;
use crate::chapter9::structh_with_field::{new_map, GrayscaleMap, broom_test};

use crate::chapter11::trait_object_basic::hello;
use crate::chapter11::generic_and_type_paramter::top_ten;
use crate::chapter11::trait_and_self::{do_it};
use crate::chapter11::trait_default_method::default_call;


mod chapter9;
mod chapter11;


// 필드가 모두 public인 경우에는 아래와 같이 assert_eq!()에서 필드로 직접 접근하고 생성할 수 있다.
/*
fn t1_1() {
    let size = (1024, 956);
    let pixels = vec![0; size.0 * size.1];
    let image = new_map(size, pixels);

    assert_eq!(image.size, size);
    assert_eq!(image.pixels.len(), 1024 * 956);
}
 */

// 필드가 private인 경우에는 다른 모듈에서는 필드 이름으로 직접 생성할 수 조차 없다.
// fn t1_2() {
//     let size = (1024, 956);
//     let pixels = vec![0; size.0 * size.1];
//     let image = GrayscaleMap{
//         size,
//         pixels
//     };
//
//     assert_eq!(image.size, size);
//     assert_eq!(image.pixels.len(), 1024 * 956);
// }



fn main() {
    // t1_1();
    // t1_2();
    // broom_test();
    // hello()
    default_call();
    do_it()


}
