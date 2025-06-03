pub mod chess;

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
