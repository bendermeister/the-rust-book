
/// Add one to the given number
/// # Example
/// ```
/// use add_one::add_one;
///
/// let result = add_one(68);
/// assert_eq!(result, 69);
/// ```
pub fn add_one(i: i32) -> i32 { i + 1 } 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(1);
        assert_eq!(result, 2);

        let result = add_one(2);
        assert_eq!(result, 3);

        let result = add_one(68);
        assert_eq!(result, 69);
    }
}
