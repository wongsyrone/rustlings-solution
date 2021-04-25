// iterators4.rs



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    /*
     * solution 1: use recursion
    if num == 1 {
        1
    } else if num == 2 {
        2
    } else {
        factorial(num - 1) * num
    }
     */

    // solution 2: use iterator
    // (1..=num).product()  // type 1
    (1..num + 1).product()  // type 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
