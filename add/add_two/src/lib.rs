

/// Add two to a given `i32`
///
/// # Example
/// ```
/// use add_two;
///
/// let result = add_two::add_two(67);
/// assert_eq!(result, 69);
/// ```
pub fn add_two(i: i32) -> i32 {
    i + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(1);
        assert_eq!(result, 3);

        let result = add_two(2);
        assert_eq!(result, 4);

        let result = add_two(67);
        assert_eq!(result, 69);
    }
}
