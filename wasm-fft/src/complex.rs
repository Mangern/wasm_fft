use std::ops::{
    Add, Sub, Mul, AddAssign,
};

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    real: f64,
    imag: f64
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}


impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
    }
}

impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Complex { real, imag: 0.0 }
    }
}

impl Complex {
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Complex {
            real: r * theta.cos(),
            imag: r * theta.sin()
        }
    }
}
