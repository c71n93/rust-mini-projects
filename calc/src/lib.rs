use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, thiserror::Error)]
pub enum CalculationError {
    #[error("Error! Integer overflow. {0}")]
    IntegerOverflow(String),
    #[error("Error! Division by zero. {0}")]
    DivisionByZero(String),
}

pub struct Calc {
    reg: Result<i32, CalculationError>,
}

impl Calc {
    pub fn new(x: i32) -> Self {
        Self { reg: Ok(x) }
    }

    pub fn add(mut self, rhs: i32) -> Self {
        self.reg = self.reg.and_then(|reg| {
            reg.checked_add(rhs)
                .ok_or(CalculationError::IntegerOverflow(String::from(
                    "Occurred in 'add'",
                )))
        });
        self
    }

    pub fn sub(mut self, rhs: i32) -> Self {
        self.reg = self.reg.and_then(|reg| {
            reg.checked_sub(rhs)
                .ok_or(CalculationError::IntegerOverflow(String::from(
                    "Occurred in 'sub'",
                )))
        });
        self
    }

    pub fn mul(mut self, rhs: i32) -> Self {
        self.reg = self.reg.and_then(|reg| {
            reg.checked_mul(rhs)
                .ok_or(CalculationError::IntegerOverflow(String::from(
                    "Occurred in 'mul'",
                )))
        });
        self
    }

    pub fn div(mut self, rhs: i32) -> Self {
        self.reg = self.reg.and_then(|reg| {
            reg.checked_div(rhs).ok_or_else(|| {
                let msg = "Occurred in 'div'";
                if rhs == 0 {
                    CalculationError::DivisionByZero(String::from(msg))
                } else {
                    CalculationError::IntegerOverflow(String::from(msg))
                }
            })
        });
        self
    }

    fn self_op(self, rhs: Self, op: fn(Self, i32) -> Self) -> Self {
        match rhs.reg {
            Ok(x) => op(self, x),
            Err(rhs_err) => Self {
                reg: match self.reg {
                    Ok(_) => Err(rhs_err),
                    Err(self_err) => Err(self_err),
                },
            },
        }
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.reg {
            Ok(r) => write!(f, "Result: {r}"),
            Err(e) => write!(f, "{}", e),
        }
    }
}

impl Default for Calc {
    fn default() -> Self {
        Self { reg: Ok(0) }
    }
}

impl Add for Calc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.self_op(rhs, Calc::add)
    }
}

impl Sub for Calc {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.self_op(rhs, Calc::sub)
    }
}

impl Mul for Calc {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.self_op(rhs, Calc::mul)
    }
}

impl Div for Calc {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.self_op(rhs, Calc::div)
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
        assert_eq!(
            "Error! Integer overflow. Occurred in 'add'",
            res.to_string()
        );
    }

    #[test]
    fn division_by_zero() {
        let res = Calc::default().div(0);
        assert_eq!(
            "Error! Division by zero. Occurred in 'div'",
            res.to_string()
        );
    }

    #[test]
    fn merge_error_first() {
        let c1 = Calc::default().add(i32::MAX).mul(2);
        let c2 = c1 - Calc::default().div(0);
        assert_eq!("Error! Integer overflow. Occurred in 'mul'", c2.to_string())
    }

    #[test]
    fn merge_error_second() {
        let c1 = Calc::default().add(1);
        let c2 = c1 * Calc::default().div(0);
        assert_eq!("Error! Division by zero. Occurred in 'div'", c2.to_string())
    }

    #[test]
    fn operators() {
        let a = 2;
        let b = 3;
        let c = 4;
        let d = 2;
        let actual = (a + b) * c / d;
        let res = (Calc::new(a) + Calc::new(b)) * Calc::new(c) / Calc::new(d);
        assert_eq!(format!("Result: {actual}"), res.to_string())
    }
}
