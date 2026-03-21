pub mod chess;

/// Adds two numbers together.
///
/// This function computes the sum f(a, b) = a + b
///
/// cargo doc will render everything inside src/ that belongs to the library crate
///
/// this will be rendered as markdown in the browser by rustdoc crate, bellow is a seperator and then a header
///
/// ---
///
/// this phrase apears rendered because it is a doc
//  this one is just a code comment, not documentation
/// # Example
/// ```
// it is normal for the rust analyzer to give this error in the doc test
/// use franiquilafish::add;
/// let result = add(2, 2); // rust comment apearing in the markdown
/// assert_eq!(result, 4);
///
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn _it_works() {
        let result = add(2, 2);
        dbg!(result);
        println!("{}", result);
        assert_eq!(result, 4);
    }
}
