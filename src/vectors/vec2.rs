use super::GenericVector;

#[derive(PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl GenericVector for Vec2 {
    fn cross(self, other: Vec2) -> Self {
        other
    }
    fn dot(self, other: Vec2) -> Self {
        other
    }

    fn vadd(self, other: Vec2) -> Self {
        todo!()
    }

    fn add_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vsub(self, other: Vec2) -> Self {
        todo!()
    }

    fn sub_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vdiv(self, other: Vec2) -> Self {
        todo!()
    }

    fn div_f64(self, by: f64) -> Self {
        todo!()
    }

    fn vmul(self, other: Vec2) -> Self {
        todo!()
    }

    fn all_eq(self) -> bool {
        todo!()
    }

    fn mul_f64(self, by: f64) -> Self {
        Self {
            x: self.x * by,
            y: self.y * by,
        }
    }
}

crate::impl_common_ops_for_vec!(Vec2);
