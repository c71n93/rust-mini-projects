use std::fmt::{Display, Formatter};
use crate::calc_errors::CalculationError;

pub mod calc_errors {
    pub enum CalculationError {
        IntegerOverflow(String),
        DivisionByZero(String),
    }
}

pub struct Calc {
    reg: Result<i32, CalculationError>
}

impl Calc {
    // TODO: To fix copy-paste and lots of boilerplate code in every method.
    pub fn add(mut self, rhs: i32) -> Self {
        match self.reg {
            Ok(x) => self.reg = x.checked_add(rhs).ok_or(
                CalculationError::IntegerOverflow(String::from("Occurred in 'add'"))
            ),
            Err(e) => self.reg = Err(e)
        }
        self
    }

    pub fn sub(mut self, rhs: i32) -> Self {
        match self.reg {
            Ok(x) => self.reg = x.checked_sub(rhs).ok_or(
                CalculationError::IntegerOverflow(String::from("Occurred in 'sub'"))
            ),
            Err(e) => self.reg = Err(e)
        }
        self
    }

    pub fn mul(mut self, rhs: i32) -> Self {
        match self.reg {
            Ok(x) => self.reg = x.checked_mul(rhs).ok_or(
                CalculationError::IntegerOverflow(String::from("Occurred in 'mul'"))
            ),
            Err(e) => self.reg = Err(e)
        }
        self
    }

    pub fn div(mut self, rhs: i32) -> Self {
        match self.reg {
            Ok(x) => self.reg = x.checked_div(rhs).ok_or_else( || {
                    let msg = "Occurred in 'div'";
                    if rhs == 0 {
                        CalculationError::DivisionByZero(String::from(msg))
                    } else {
                        CalculationError::IntegerOverflow(String::from(msg))
                    }
                }
            ),
            Err(e) => self.reg = Err(e)
        }
        self
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.reg {
            Ok(r) => write!(f, "Result: {r}"),
            Err(e) => match e {
                CalculationError::IntegerOverflow(msg) => write!(
                    f,
                    "Error! Integer overflow. {msg}"
                ),
                CalculationError::DivisionByZero(msg) => write!(
                    f,
                    "Error! Division by zero. {msg}"
                )
            }
        }
    }
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            reg: Ok(0)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Calc;

    #[test]
    fn simple_addition() {
        let res = Calc::default().add(2).add(2);
        assert_eq!("Result: 4", res.to_string());
    }

    #[test]
    fn overflow() {
        let res = Calc::default().add(i32::MAX).add(1);
        assert_eq!("Error! Integer overflow. Occurred in 'add'", res.to_string());
    }

    #[test]
    fn division_by_zero() {
        let res = Calc::default().div(0);
        assert_eq!("Error! Division by zero. Occurred in 'div'", res.to_string());
    }
}

