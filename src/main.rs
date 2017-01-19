#[cfg(test)]
#[macro_use]
extern crate quickcheck;

fn add_me(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
    println!("2 + 2 = {}", add_me(2, 2));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add_me() {
        use super::add_me;
        assert!(add_me(2, 2) == 4);
        assert!(add_me(2, -7) == -5);
    }

    quickcheck! {
        fn prop(x: i32, y: i32) -> bool {
            use super::add_me;
            add_me(x, y) == x + y
        }
    }
}
