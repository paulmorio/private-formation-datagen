use core::fmt::Debug;
use num::Zero;
use serde::Deserialize;
use serde::Serialize;
use std::ops::*;

pub trait Vector:
    PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<f64>
    + DivAssign<f64>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + PartialOrd
    + Sized
    + Copy
    + Zero
    + Debug
    + Serialize
    + for<'a> Deserialize<'a>
{
    fn length(&self) -> f64;

    fn repr_length() -> usize;

    fn repr(&self) -> Vec<f64>;
}

pub type Seconds = f64;
pub type Metres = f64;
pub type MetresPerSecond = f64;
pub type Radians = f64;
pub type RadiansPerSecond = f64;

impl Vector for f64 {
    fn length(&self) -> f64 {
        *self
    }

    fn repr_length() -> usize {
        1
    }

    fn repr(&self) -> Vec<f64> {
        vec![*self]
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Metres2D {
    pub x: f64,
    pub y: f64,
}

pub type Metres2DPerSecond = Metres2D;

impl Add for Metres2D {
    type Output = Metres2D;

    fn add(self, rhs: Metres2D) -> Metres2D {
        Metres2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Metres2D {
    type Output = Metres2D;

    fn sub(self, rhs: Metres2D) -> Metres2D {
        Metres2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Metres2D {
    type Output = Metres2D;

    fn mul(self, rhs: f64) -> Metres2D {
        Metres2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Metres2D {
    type Output = Metres2D;

    fn div(self, rhs: f64) -> Metres2D {
        Metres2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Zero for Metres2D {
    fn zero() -> Self {
        Metres2D { x: 0., y: 0. }
    }

    fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0.
    }
}

impl AddAssign for Metres2D {
    fn add_assign(&mut self, rhs: Metres2D) {
        *self = *self + rhs;
    }
}

impl SubAssign for Metres2D {
    fn sub_assign(&mut self, rhs: Metres2D) {
        *self = *self - rhs;
    }
}

impl MulAssign<f64> for Metres2D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl DivAssign<f64> for Metres2D {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Vector for Metres2D {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn repr_length() -> usize {
        2
    }

    fn repr(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}

pub struct PolarMetres2D {
    pub theta: f64,
    pub r: f64,
}

impl PolarMetres2D {
    pub fn to_cartesian(&self) -> Metres2D {
        Metres2D {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    pub fn new(r: f64, theta: f64) -> PolarMetres2D {
        PolarMetres2D { r, theta }
    }
}

impl Metres2D {
    pub fn to_polar(&self) -> PolarMetres2D {
        PolarMetres2D {
            theta: self.y.atan2(self.x),
            r: self.length(),
        }
    }

    pub fn new(x: f64, y: f64) -> Metres2D {
        Metres2D { x, y }
    }

    pub fn angle(&self) -> Radians {
        self.to_polar().theta
    }
}

#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct OrientedPosition2D {
    #[serde(flatten)]
    pub position: Metres2D,
    /// Anticlockwise rotation
    #[serde(rename = "r")]
    pub rotation: Radians,
}

impl OrientedPosition2D {
    pub fn new(x: f64, y: f64, theta: f64) -> OrientedPosition2D {
        OrientedPosition2D {
            position: Metres2D::new(x, y),
            rotation: theta,
        }
    }
}
