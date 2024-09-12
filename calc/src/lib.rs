use std::fmt::{Display, Formatter};

pub struct Calc {
    reg: Option<i32>
}

//TODO: figure out something with pretty pattern matching.
impl Calc {
    pub fn add(mut self, rhs: i32) -> Self {
        match self.reg {
            Some(x) => self.reg = x.checked_add(rhs),
            None => self.reg = None
        }
        self
    }

    pub fn sub(mut self, rhs: i32) -> Self {
        match self.reg {
            Some(x) => self.reg = x.checked_sub(rhs),
            None => self.reg = None
        }
        self
    }

    pub fn mul(mut self, rhs: i32) -> Self {
        match self.reg {
            Some(x) => self.reg = x.checked_mul(rhs),
            None => self.reg = None
        }
        self
    }

    pub fn div(mut self, rhs: i32) -> Self {
        match self.reg {
            Some(x) => self.reg = x.checked_div(rhs),
            None => self.reg = None
        }
        self
    }
}

impl Display for Calc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reg {
            Some(r) => write!(f, "Result: {}", r),
            None => write!(f, "Error")
        }
    }
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            reg: Some(0)
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
        assert_eq!("Error", res.to_string());
    }

    #[test]
    fn division_by_zero() {
        let res = Calc::default().div(0);
        assert_eq!("Error", res.to_string());
    }
}

