#[derive(PartialEq)]
enum O {
    A = 0,
    B,
    C,
}
fn main() {
    assert_eq!(O::A as i32, 0);
    assert_eq!(O::B as i32, 1);
    assert_eq!(O::C as i32, 2);
}
