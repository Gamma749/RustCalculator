mod operations;
use core::fmt;
use operations::{generate_operations, Operation};
use std::collections::HashMap;

pub struct Calculator {
    stack: Vec<f64>,
    operations: HashMap<&'static str, Operation>,
}

impl fmt::Debug for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.stack)
    }
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            stack: Vec::new(),
            operations: generate_operations(),
        }
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value)
    }

    pub fn perform_operation(&mut self, operation_str: &str) -> Result<(), &'static str> {
        let op = match self.operations.get(operation_str) {
            None => {
                return Err("Specified operation does not exist!");
            }
            Some(boxed_op) => *boxed_op,
        };

        op(&mut self.stack)
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    // BASIC TESTS --------------------------------------------------------------------------------
    #[test]
    fn calculator_init() {
        Calculator::new();
    }

    #[test]
    fn calculator_push() {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(10.0);
        calc.push(-1.0);
        calc.push(3.14);
        calc.push(std::f64::MAX);
    }

    #[test]
    fn calculator_pop() {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(10.0);
        calc.push(-1.0);
        calc.push(3.14);
        calc.push(std::f64::MAX);

        calc.pop();
        calc.pop();
        calc.pop();
        calc.pop();
    }

    // ADDITION TESTS -----------------------------------------------------------------------------

    #[test]
    fn basic_add_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(1.0);
        calc.perform_operation("+")?;
        let result = calc.pop().unwrap();
        assert_eq!(2.0, result);
        Ok(())
    }

    #[test]
    fn negative_add_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(10.0);
        calc.push(-12.0);
        calc.perform_operation("+")?;
        let result = calc.pop().unwrap();
        assert_eq!(-2.0, result);
        Ok(())
    }

    #[test]
    fn large_add_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(123456789.0);
        calc.push(987654321.0);
        calc.perform_operation("+")?;
        let result = calc.pop().unwrap();
        assert_eq!(1111111110.0, result);
        Ok(())
    }

    #[test]
    fn floating_add_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(3.1459);
        calc.push(2.7182);
        calc.perform_operation("+")?;
        let result = calc.pop().unwrap();
        assert_eq!(3.1459 + 2.7182, result);
        Ok(())
    }

    #[test]
    fn float_limits_add_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(std::f64::MAX);
        calc.push(std::f64::MAX);
        calc.perform_operation("+")?;
        let result = calc.pop().unwrap();
        assert_eq!(std::f64::MAX + std::f64::MAX, result);
        Ok(())
    }

    // SUBTRACTION TESTS --------------------------------------------------------------------------

    #[test]
    fn basic_sub_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(1.0);
        calc.perform_operation("-")?;
        let result = calc.pop().unwrap();
        assert_eq!(0.0, result);
        Ok(())
    }

    #[test]
    fn negative_sub_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(10.0);
        calc.push(-12.0);
        calc.perform_operation("-")?;
        let result = calc.pop().unwrap();
        assert_eq!(22.0, result);
        Ok(())
    }

    #[test]
    fn large_sub_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(123456789.0);
        calc.push(987654321.0);
        calc.perform_operation("-")?;
        let result = calc.pop().unwrap();
        assert_eq!(123456789.0 - 987654321.0, result);
        Ok(())
    }

    #[test]
    fn floating_sub_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(3.1459);
        calc.push(2.7182);
        calc.perform_operation("-")?;
        let result = calc.pop().unwrap();
        assert_eq!(3.1459 - 2.7182, result);
        Ok(())
    }

    #[test]
    fn float_limits_sub_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(std::f64::MIN);
        calc.push(std::f64::MAX);
        calc.perform_operation("-")?;
        let result = calc.pop().unwrap();
        assert_eq!(std::f64::MIN - std::f64::MAX, result);
        Ok(())
    }

    // MULTIPLICATION TESTS --------------------------------------------------------------------------

    #[test]
    fn basic_mul_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(1.0);
        calc.perform_operation("*")?;
        let result = calc.pop().unwrap();
        assert_eq!(1.0, result);
        Ok(())
    }

    #[test]
    fn negative_mul_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(10.0);
        calc.push(-12.0);
        calc.perform_operation("*")?;
        let result = calc.pop().unwrap();
        assert_eq!(-120.0, result);
        Ok(())
    }

    #[test]
    fn large_mul_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(123456789.0);
        calc.push(987654321.0);
        calc.perform_operation("*")?;
        let result = calc.pop().unwrap();
        assert_eq!(123456789.0 * 987654321.0, result);
        Ok(())
    }

    #[test]
    fn floating_mul_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(3.1459);
        calc.push(2.7182);
        calc.perform_operation("*")?;
        let result = calc.pop().unwrap();
        assert_eq!(3.1459 * 2.7182, result);
        Ok(())
    }

    #[test]
    fn float_limits_mul_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(std::f64::MIN);
        calc.push(std::f64::MAX);
        calc.perform_operation("*")?;
        let result = calc.pop().unwrap();
        assert_eq!(std::f64::MIN * std::f64::MAX, result);
        Ok(())
    }

    // DIVISION TESTS --------------------------------------------------------------------------

    #[test]
    fn basic_div_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(1.0);
        calc.perform_operation("/")?;
        let result = calc.pop().unwrap();
        assert_eq!(1.0, result);
        Ok(())
    }

    #[test]
    fn negative_div_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(10.0);
        calc.push(-12.0);
        calc.perform_operation("/")?;
        let result = calc.pop().unwrap();
        assert_eq!(10.0 / -12.0, result);
        Ok(())
    }

    #[test]
    fn large_div_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(123456789.0);
        calc.push(987654321.0);
        calc.perform_operation("/")?;
        let result = calc.pop().unwrap();
        assert_eq!(123456789.0 / 987654321.0, result);
        Ok(())
    }

    #[test]
    fn floating_div_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(3.1459);
        calc.push(2.7182);
        calc.perform_operation("/")?;
        let result = calc.pop().unwrap();
        assert_eq!(3.1459 / 2.7182, result);
        Ok(())
    }

    #[test]
    fn float_limits_div_test() -> Result<(), &'static str> {
        let mut calc = Calculator::new();
        calc.push(std::f64::MIN);
        calc.push(std::f64::MAX);
        calc.perform_operation("/")?;
        let result = calc.pop().unwrap();
        assert_eq!(std::f64::MIN / std::f64::MAX, result);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "DIVISION BY ZERO")]
    #[allow(unused_must_use)]
    fn div_by_zero_test(){
        let mut calc = Calculator::new();
        calc.push(1.0);
        calc.push(0.0);
        calc.perform_operation("/");
    }
}
