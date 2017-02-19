use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
struct Complex {
    r: f64,
    i: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Complex{ r, i: 0. } => write!(f, "{}", r),
            Complex{ r: 0., i } => write!(f, "{}", i),
            Complex{ r, i } => write!(f, "({} + {}I)", r, i),
        }
    }
}

impl Complex {
    #[allow(dead_code)]
    fn conjugate(self) -> Complex {
        Complex {
            r: self.r,
            i: -self.i,
        }
    }
    fn inverse(self) -> Complex {
        let d = self.r*self.r + self.i*self.i;
        Complex {
            r: self.r / d,
            i: -self.i / d,
        }
    }
}

impl ops::Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl ops::Add<Complex> for f64 {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            r: self + other.r,
            i: other.i,
        }
    }
}

impl ops::Add<f64> for Complex {
    type Output = Complex;
    fn add(self, other: f64) -> Complex {
        Complex {
            r: self.r + other,
            i: self.i,
        }
    }
}

impl ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }
}

impl ops::Sub<Complex> for f64 {
    type Output = Complex;
    fn sub(self, other: Complex) -> Complex {
        Complex {
            r: self - other.r,
            i: other.i,
        }
    }
}

impl ops::Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, other: f64) -> Complex {
        Complex {
            r: self.r - other,
            i: self.i,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            r: self.r*other.r - self.i*other.i,
            i: self.r*other.i + self.i*other.r,
        }
    }
}

impl ops::Mul<Complex> for f64 {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            r: self * other.r,
            i: self * other.i,
        }
    }
}

impl ops::Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, other: f64) -> Complex {
        Complex {
            r: self.r * other,
            i: self.i * other,
        }
    }
}

impl ops::Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        self * other.inverse()
    }
}

impl ops::Div<Complex> for f64 {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let other = other.inverse();
        Complex {
            r: self * other.r,
            i: self * other.i,
        }
    }
}

impl ops::Div<f64> for Complex {
    type Output = Complex;
    fn div(self, other: f64) -> Complex {
        Complex {
            r: self.r / other,
            i: self.i / other,
        }
    }
}

const I: Complex = Complex{ r: 0., i: 1. };

fn main() {
    let x = 1. + I;
    println!("{}", x);
    let y = I + 1.;
    println!("{}", y);
    println!("{}", x - y);
    println!("{}", -y);
    let z = x*I;
    println!("{}", z);
    println!("{}", 3.*x);
    println!("{}", x.inverse());
    println!("{}", x*x.inverse());
    println!("{}", x / (3.*x));
}
