fn add_me(x: i32, y: i32) -> i32 {
    x * y
}

#[test]
fn test_add_me() {
    assert!(add_me(2, 2) == 4);
}

fn main() {
    println!("Hello, world!");
    println!("2 + 2 = {}", add_me(2, 2));
}
