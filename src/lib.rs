pub mod chess;


/// Adds two numbers together.
/// 
/// This function computes the sum f(a, b) = a + b
/// 
/// # Example
/// ```
/// use franiquilafish::add;
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// 
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        dbg!(result);
        println!("{}", result);
        assert_eq!(result, 4);
    }
}
