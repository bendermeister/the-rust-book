pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn panic_func() {
    panic!("PANIC!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 3,
        };
        assert!(
            larger.can_hold(&smaller),
            "This is how to put message in assert: {larger:?}"
        );
    }

    #[test]
    #[should_panic(expected = "PANIC!")]
    fn test_panic() {
        panic_func();
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
