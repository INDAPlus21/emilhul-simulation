use std::ops;
use rand::Rng;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector2{
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn from(a: f32, b: f32) -> Self {
        Self {
            x: a,
            y: b,
        }
    }

    pub fn from_angle(angle: f32) -> Self {
        let mut a: f32 = angle.cos();
        if a < 0.000001
            && a > -0.000001 {
            a = 0.0;
        }
        let mut b = angle.sin();
        if  b < 0.000001
        && b > -0.000001 {
            b = 0.0;
        }
        Self {
            x: a,
            y: b,
        }
    }

    pub fn random() -> Self {
        let a: f32 = rand::thread_rng().gen_range(-1.0..1.0);
        let b: f32 = rand::thread_rng().gen_range(-1.0..1.0);
        Self {
            x: a,
            y: b,
        }
    }

    pub fn angle(self) -> f32 {
        (self.y/self.x).atan()
    }

    pub fn angle_to(self, other: Vector2) -> f32 {
        ((self * other)/(self.normal() * other.normal())).acos()
    }

    pub fn normal(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalized(&mut self) -> Self {
        self.x *= 1.0/self.normal();
        self.y *= 1.0/self.normal();
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul for Vector2 {
    type Output = f32;

    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI as PI;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4)
    }

    #[test]
    fn vector2_new() {
        let a = Vector2 {
            x: 0.0,
            y: 0.0,
        }; 
        assert_eq!(a, Vector2::new())
    }

    #[test]
    fn vector2_from() {
        let a = Vector2 {
            x: 10.0,
            y: -5.2,
        }; 
        assert_eq!(a, Vector2::from(10.0, -5.2))
    }

    #[test]
    fn vector2_add() {
        let a = Vector2::from(4.0, 3.0);
        let b = Vector2::from(2.0, -1.0);
        let c = Vector2::from(6.0, 2.0);

        assert_eq!(c, a + b)
    }

    #[test]
    fn vector2_sub() {
        let a = Vector2::from(4.0, 3.0);
        let b = Vector2::from(2.0, -1.0);
        let c = Vector2::from(2.0, 4.0);

        assert_eq!(c, a - b)
    }

    #[test]
    fn vector2_scalar_mul() {
        let a = Vector2::from(4.0, 3.0);
        let b = Vector2::from(2.0, -1.0);
        let c: f32 = 5.0;

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector2_vec_const_mul() {
        let a = Vector2::from(4.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector2::from(2.0, 1.0);

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector2_const_vec_mul() {
        let a = Vector2::from(4.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector2::from(2.0, 1.0);
        assert_eq!(c, b * a)
    }

    #[test]
    fn vector2_normal() {
        let a = Vector2::from(4.0, 3.0);
        assert_eq!(a.normal(), 5.0);
    }

    #[test]
    fn vector2_normalized() {
        let mut a = Vector2::from(2.0, 0.0);
        let b = Vector2::from(1.0, 0.0);
        a.normalized();
        assert_eq!(a, b)
    }

    #[test]
    fn vector2_angle_to() {
        let a = Vector2::from(1.0, 0.0);
        let b = Vector2::from_angle(PI/2.0);
        assert_eq!(a.angle_to(b), PI/2.0);
    }

    #[test]
    fn vector2_angle() {
        let a = Vector2::from_angle(PI/2.0);
        assert_eq!(a.angle(), PI/2.0);
    }
}