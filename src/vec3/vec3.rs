use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
 
use crate::common;
 
#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}
 
impl Vec3 {
    pub fn axis_x(&self) -> Self {
        Self::new(self.x() + 1.0, self.y(), self.z())
    }

    pub fn axis_y(&self) -> Self {
        Self::new(self.x(), self.y() + 1.0, self.z())
    }

    pub fn axis_z(&self) -> Self {
        Self::new(self.x(), self.y(), self.z() + 1.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
 
    pub fn random() -> Vec3 {
        Vec3::new(
            common::random_double(),
            common::random_double(),
            common::random_double(),
        )
    }
 
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            common::random_double_range(min, max),
            common::random_double_range(min, max),
            common::random_double_range(min, max),
        )
    }
   

    /// Rotate self around the point p on a definined axis and with a defined angle
    pub fn rotate_around(&mut self, p: Point3, axis: Vec3, angle: f64) {
        // Calculer le vecteur entre le point p et le vecteur actuel
        let mut dir = *self - p;
        
        // Appliquer la rotation sur ce vecteur dir (fonction de rotation autour de l'axe)
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        let dot = dot(dir, axis);  // Produit scalaire entre le vecteur et l'axe
        let cross = cross(axis, dir);  // Produit vectoriel entre l'axe et le vecteur

        dir = dir * cos_angle + cross * sin_angle + axis * dot * (1.0 - cos_angle);
        
        // Réassigner la nouvelle position
        *self = p + dir;
    }
    
 
    pub fn x(&self) -> f64 {
        self.e[0]
    }
 
    pub fn y(&self) -> f64 {
        self.e[1]
    }
 
    pub fn z(&self) -> f64 {
        self.e[2]
    }
 
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
 
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}
 
// Type alias
pub type Point3 = Vec3;
 
// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
 
// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;
 
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
 
// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}
 
// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
 
// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}
 
// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
 
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}
 
// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}
 
// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}
 
// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}
 
// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}
 

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}
 
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}
 
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
 
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
 
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}


pub fn rotate_vector(v: Vec3, axis: Vec3, angle: f64) -> Vec3 {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let unit_axis = unit_vector(axis);

    v * cos_theta
        + cross(unit_axis, v) * sin_theta
        + unit_axis * dot(unit_axis, v) * (1.0 - cos_theta)
}