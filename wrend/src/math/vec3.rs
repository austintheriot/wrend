use crate::math;
use crate::Matrix4x4;
use std::f64::consts::PI;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

/// Color and position values are stored as floats
/// Colors are actually *written* to memory as bytes (0->255)
#[derive(Clone, PartialEq, Copy, Debug, Default, PartialOrd)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn new_with_tuple(tuple: (f64, f64, f64)) -> Self {
        Vec3(tuple.0, tuple.1, tuple.2)
    }

    pub fn new_with_array(array: [f64; 3]) -> Self {
        Vec3(array[0], array[1], array[2])
    }

    pub fn splat(value: f64) -> Self {
        Vec3(value, value, value)
    }

    pub fn to_f64_array(&self) -> [f64; 3] {
        self.into()
    }

    pub fn to_f32_array(&self) -> [f32; 3] {
        self.into()
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }

    pub fn normalize(self: Vec3) -> Vec3 {
        self / self.length()
    }

    /// from (-1, 1) to (0, 1)
    pub fn map_to_color_range(&self) -> Vec3 {
        (1. + *self) * 0.5
    }

    pub fn random_with_range(min: f64, max: f64) -> Self {
        Vec3(
            math::random_with_range(min, max),
            math::random_with_range(min, max),
            math::random_with_range(min, max),
        )
    }

    /// INSIDE the unit sphere
    pub fn random_point_in_unit_sphere() -> Vec3 {
        let u = js_sys::Math::random();
        let v = js_sys::Math::random();
        let theta = u * 2.0 * PI;
        let phi = (2.0 * v - 1.0).acos();
        let r = (js_sys::Math::random()).cbrt();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let x = r * sin_phi * cos_theta;
        let y = r * sin_phi * sin_theta;
        let z = r * cos_phi;
        Vec3(x, y, z)
    }

    /// along the EDGE of the unit sphere
    pub fn random_unit_vector() -> Vec3 {
        Vec3::normalize(Vec3::random_point_in_unit_sphere())
    }

    pub fn is_near_zero(&self) -> bool {
        let threshold = 1e-10;
        self.x() < threshold && self.y() < threshold && self.z() < threshold
    }
}

impl From<Vec3> for [f64; 3] {
    fn from(vec3: Vec3) -> Self {
        [vec3.0, vec3.1, vec3.2]
    }
}

impl From<&Vec3> for [f64; 3] {
    fn from(vec3: &Vec3) -> Self {
        [vec3.0, vec3.1, vec3.2]
    }
}


impl From<Vec3> for [f32; 3] {
    fn from(vec3: Vec3) -> Self {
        [vec3.0 as f32, vec3.1 as f32, vec3.2 as f32]
    }
}

impl From<&Vec3> for [f32; 3] {
    fn from(vec3: &Vec3) -> Self {
        [vec3.0 as f32, vec3.1 as f32, vec3.2 as f32]
    }
}


impl From<Vec3> for (f64, f64, f64) {
    fn from(vec3: Vec3) -> Self {
        (vec3.0, vec3.1, vec3.2)
    }
}


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(self.0 * -1., self.1 * -1., self.2 * -1.)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.0;
        self.2 /= rhs.0;
    }
}

impl<V: Into<Vec3>> Add<V> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        self + *rhs
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        *self + *rhs
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        *self + rhs
    }
}

impl Add<&f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &f64) -> Self::Output {
        self + *rhs
    }
}

impl Add<&f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &f64) -> Self::Output {
        *self + *rhs
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self + rhs.0, self + rhs.1, self + rhs.2)
    }
}

impl Add<&Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        self + *rhs
    }
}

impl Add<Vec3> for &f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}

impl Add<&Vec3> for &f64 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        *self + *rhs
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self - *rhs
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        *self - *rhs
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Sub<&f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &f64) -> Self::Output {
        self - *rhs
    }
}

impl Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        *self - rhs
    }
}

impl Sub<&f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &f64) -> Self::Output {
        *self - *rhs
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 - self, rhs.1 - self, rhs.2 - self)
    }
}

impl Sub<&Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        *rhs - self
    }
}

impl Sub<Vec3> for &f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        rhs - *self
    }
}

impl Sub<&Vec3> for &f64 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        *rhs - *self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        self * *rhs
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        *self * *rhs
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<&f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        self * *rhs
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        *self * rhs
    }
}

impl Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        *self * *rhs
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * self
    }
}

impl Mul<Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * *self
    }
}

impl Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * *self
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        *self / rhs
    }
}

impl Div<&Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        self / *rhs
    }
}

impl Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        *self / *rhs
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        *self / rhs
    }
}

impl Div<&f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &f64) -> Self::Output {
        self / *rhs
    }
}

impl Div<&f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &f64) -> Self::Output {
        *self / *rhs
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 / self, rhs.1 / self, rhs.2 / self)
    }
}

impl Div<Vec3> for &f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / *self
    }
}

impl Div<&Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        *rhs / self
    }
}

impl Div<&Vec3> for &f64 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        *rhs / *self
    }
}

impl From<Matrix4x4> for Vec3 {
    fn from(m4: Matrix4x4) -> Self {
        Vec3(m4.0[3], m4.0[7], m4.0[11])
    }
}

impl From<&Matrix4x4> for Vec3 {
    fn from(m4: &Matrix4x4) -> Self {
        (*m4).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod vec3_tests {
        use super::*;

        #[test]
        fn test_add() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(0.0, 2.0, 4.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a + b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a + b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a + b, result);
            }
        }

        #[test]
        fn test_sub() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(0.0, 0.0, 0.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a - b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a - b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a - b, result);
            }
        }

        #[test]
        fn test_mul() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(0.0, 1.0, 4.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a * b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a * b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a * b, result);
            }
        }

        #[test]
        fn test_div() {
            let a = Vec3(0.0, 1.0, 6.0);
            let b = Vec3(1.0, 2.0, 3.0);
            let result = Vec3(0.0, 0.5, 2.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a / b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a / b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a / b, result);
            }
        }
    }

    #[cfg(test)]
    mod scalar_right_tests {
        use super::*;

        #[test]
        fn test_add() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = 1.0;
            let result = Vec3(1.0, 2.0, 3.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a + b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a + b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a + b, result);
            }
        }

        #[test]
        fn test_sub() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = 1.0;
            let result = Vec3(-1.0, 0.0, 1.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a - b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a - b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a - b, result);
            }
        }

        #[test]
        fn test_mul() {
            let a = Vec3(0.0, 1.0, 2.0);
            let b = 3.0;
            let result = Vec3(0.0, 3.0, 6.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a * b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a * b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a * b, result);
            }
        }

        #[test]
        fn test_div() {
            let a = Vec3(0.0, 3.0, 6.0);
            let b = 3.0;
            let result = Vec3(0.0, 1.0, 2.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a / b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a / b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a / b, result);
            }
        }
    }

    #[cfg(test)]
    mod scalar_left_tests {
        use super::*;

        #[test]
        fn test_add() {
            let a = 1.0;
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(1.0, 2.0, 3.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a + b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a + b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a + b, result);
            }
        }

        #[test]
        fn test_sub() {
            let a = 1.0;
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(-1.0, 0.0, 1.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a - b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a - b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a - b, result);
            }
        }

        #[test]
        fn test_mul() {
            let a = 3.0;
            let b = Vec3(0.0, 1.0, 2.0);
            let result = Vec3(0.0, 3.0, 6.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a * b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a * b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a * b, result);
            }
        }

        #[test]
        fn test_div() {
            let a = 3.0;
            let b = Vec3(0.0, 3.0, 6.0);
            let result = Vec3(0.0, 1.0, 2.0);

            {
                let a = &a;
                let b = b;
                assert_eq!(a / b, result);
            }

            {
                let a = a;
                let b = &b;
                assert_eq!(a / b, result);
            }

            {
                let a = &a;
                let b = &b;
                assert_eq!(a / b, result);
            }
        }
    }
}
