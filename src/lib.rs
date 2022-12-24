
//! # Math Libary
//! 
//! 'Math Libary' is a collection of utilities to perfom simple
//! math calculations.

/// Adds numbers together.
/// 
/// Example
/// 
/// ```
/// let num1 = 9;
/// let num2 = 10;
/// let answer = math_lib::add(num1, num2);
/// println!("{}", answer);
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
/// let answer = math_lib::subtract(num1, num2);
/// println!("{}", answer);
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
/// let answer = math_lib::multiply(num1, num2);
/// println!("{}", answer);
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
/// let answer = math_lib::divide(num1, num2);
/// println!("{}", answer);
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
/// let answer = math_lib::find_remainder(num1, num2);
/// println!("{}", answer);
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
