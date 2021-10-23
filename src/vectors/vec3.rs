#[macro_use]
use crate::vectors::vec_macros::*;

use super::{vec2::Vec2, GenericVector};

#[derive(PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// # Vec3.zero
    /// A zero vector (0,0,0)
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// # Vec3.one
    /// A one vector (1,1,1)
    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// # Vec3.up
    /// An up vector (0,0,1)
    pub fn up() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// # Vec3.down
    /// A down vector (0,0-1)
    pub fn down() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        }
    }

    /// # Vec3.forward
    /// A forward vector (1,0,0)
    pub fn forward() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        }
    }

    /// # Vec3.backward
    /// A backward vector (-1,0,0)
    pub fn backward() -> Self {
        Self {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// # Vec3.right
    /// A right vector(0,1,0)
    pub fn right() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// # Vec3.left
    /// A left vector(0,-1,0)
    pub fn left() -> Self {
        Self {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }
    }

    /// # Vec3.xaxis
    /// X axis vector (1,0,0)
    pub fn xaxis() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// # Vec3.yaxis
    /// A Y axis vector(0,1,0)
    pub fn yaxis() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    /// # Vec3.zaxis
    /// A Z axis vector(0,0,1)
    pub fn zaxis() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// # Vec3.filled
    /// Creates a vector filled with `n`
    ///
    /// ## Example
    /// ```rs
    /// let vec = Vec3::filed(1); // -> (1,1,1)
    /// ```
    pub fn filled(n: f64) -> Self {
        Self { x: n, y: n, z: n }
    }

    /// # Vec3.from_vec2
    /// Creates a vector from a vec2 and a z coordinate
    /// ```rs
    /// let vec2 = Vec2::new(2,4);
    /// let vec = Vec3::from_vec2(vec2, 6); // -> (2,4,6)
    /// ```
    pub fn from_vec2(vec2: Vec2, z: f64) -> Self {
        Self {
            x: vec2.x,
            y: vec2.y,
            z,
        }
    }
}
impl GenericVector for Vec3 {
    fn cross(self, other: Vec3) -> Self {
        other
    }
    fn dot(self, other: Vec3) -> Self {
        other
    }

    fn vadd(self, other: Vec3) -> Self {
        todo!()
    }

    fn add_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vsub(self, other: Vec3) -> Self {
        todo!()
    }

    fn sub_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vdiv(self, other: Vec3) -> Self {
        todo!()
    }

    fn div_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vmul(self, other: Vec3) -> Self {
        todo!()
    }

    fn all_eq(self) -> bool {
        todo!()
    }

    fn mul_f64(self, by: f64) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
            z: self.z * by,
        }
    }
}

impl From<Vec2> for Vec3 {
    fn from(v: Vec2) -> Self {
        Vec3::from_vec2(v, 0f64)
    }
}

crate::impl_common_ops_for_vec!(Vec3);
