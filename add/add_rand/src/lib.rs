use rand;

pub fn add_rand(i: i32) -> i32 {
    i + (rand::random::<i32>() % 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_rand(69);
        assert!(result > 69);
    }
}
