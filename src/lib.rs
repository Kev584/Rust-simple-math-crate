
//! # Math Crate
//! 
//! 'Math Crate' is a collection of utilities to perfom simple
//! math calculations.

/// Adds numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 9;
/// let num2 = 10;
/// let answer = math_crate::add(num1, num2);
/// assert_eq!(19);
/// ```
pub fn add(a: isize, b: isize) -> isize {
    a + b
}
/// Subtracts numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 7;
/// let num2 = 5;
/// let answer = math_crate::subtract(num1, num2);
/// assert_eq!(2);
/// ```
pub fn subtract(a: isize, b: isize) -> isize {
    a - b
}
/// Multiplies numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 9;
/// let num2 = 3;
/// let answer = math_crate::multiply(num1, num2);
/// assert_eq!(27);
/// ```
pub fn multiply(a: isize, b: isize) -> isize {
    a * b
}
/// Divides numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 7.1;
/// let num2 = 3.0;
/// let answer = math_crate::divide(num1, num2);
/// assert_eq!(2.366666666666667);
/// ```
pub fn divide(a: f64, b: f64) -> f64 {
    a / b
}
/// Finds the remainder of the numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 11;
/// let num2 = 2;
/// let answer = math_crate::find_remainder(num1, num2);
/// assert_eq!(1);
/// ```
pub fn find_remainder(a: isize, b: isize) -> isize {
    a % b
}

#[cfg(test)]

mod lib_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(2.0, 2.0);
        println!("Answer: {result}")
    }
}
