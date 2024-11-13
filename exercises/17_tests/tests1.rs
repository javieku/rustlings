// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_odd_number_should_return_false() {
        assert!(!is_even(3));
    }

    #[test]
    fn given_even_number_should_return_true() {
        assert!(is_even(2));
    }
}
